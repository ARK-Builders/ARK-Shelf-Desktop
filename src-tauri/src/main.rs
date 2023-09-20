#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod base;
mod command;
use base::Score;
use clap::Parser;
use command::Result;
use command::*;
use home::home_dir;
use lazy_static::lazy_static;
use notify_debouncer_full::{
    new_debouncer,
    notify::{RecursiveMode, Watcher, EventKind},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{ Arc, Mutex},
    time::{Duration, SystemTime}
};
use tauri::{AppHandle, Manager};
lazy_static! {
    pub static ref ARK_SHELF_WORKING_DIR: PathBuf = PathBuf::from(Cli::parse().path);
    pub static ref SCORES_PATH: PathBuf = PathBuf::from(Cli::parse().path)
        .join(".ark")
        .join("shelf")
        .join("scores");
}

#[derive(Parser, Default, Debug)]
#[clap(
    name = "ARK Shelf Desktop",
    about = "Desktop Version of ARK Shelf, put you bookmarks when surfing."
)]
struct Cli {
    #[clap(
        short,
        long,
        help = "Path to store .link file", 
        default_value_t = format!("{}/.ark-shelf",home_dir().unwrap().display())
    )]
    path: String,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GraphMetaData {
    description: Option<String>,
    title: Option<String>,
    image_url: Option<String>
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PreviewLoaded {
    url: String,
    graph: GraphMetaData,
    created_time: Option<SystemTime>
}

async fn get_preview(path: &std::path::PathBuf, manager: AppHandle) -> Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    let url = url::Url::parse(&file_content)?;
    let preview_url = format!("{}", url);
    println!("Preview url {preview_url:?}");
    let graph_preview = arklib::link::Link::get_preview(preview_url)
        .await
        .map_err(|_| CommandError::Arklib)?;
    let mut created_time = None;
    if let Ok(meta) = std::fs::metadata(path) {
        created_time = meta.created().ok();
    }
    let graph_data = GraphMetaData {
        image_url: graph_preview.image,
        title: graph_preview.title,
        description: graph_preview.description
    };  
    let preview_loaded = PreviewLoaded {
        url: url.into(),
        graph: graph_data,
        created_time
    };
    manager.emit_all("preview_loaded", preview_loaded).unwrap();
    Ok(())
}

fn init_link_watcher(path: std::path::PathBuf, handle: AppHandle) {
    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_millis(50), None, tx).unwrap();
        debouncer
            .watcher()
            .watch(&path, RecursiveMode::NonRecursive)
            .unwrap();
        debouncer
            .cache()
            .add_root(&path, RecursiveMode::NonRecursive);

        loop {
            match rx.recv() {
                Ok(Ok(events)) => {
                    events.into_iter().for_each(|event| {
                        if let EventKind::Create(_) = event.kind    {
                            event.event.paths.into_iter().for_each(|path| {
                                let manager = handle.clone();
                                tauri::async_runtime::spawn(async move {
                                    let _ = get_preview(&path, manager).await;
                                });
                            });

                        }  
                    });
                },
                Ok(Err(e)) => {
                    eprintln!("Errors on with the notifier watcher: {e:?}");
                },
                Err(_) => {
                    eprintln!("Error with Watcher channel!");
                    break;
                }
            }
        }
    });
}

fn main() {
    let cli = Cli::parse();

    std::fs::create_dir_all(ARK_SHELF_WORKING_DIR.as_path().join(".ark").join("shelf")).unwrap();
    lazy_static::initialize(&ARK_SHELF_WORKING_DIR);
    lazy_static::initialize(&SCORES_PATH);
    // Check if the scores file is existed, otherwise create one.
    let mut scores_file = File::options()
        .read(true)
        .open(SCORES_PATH.as_path())
        .unwrap_or_else(|_| File::create(SCORES_PATH.as_path()).unwrap());

    let mut scores_string = String::new();
    let mut scores = vec![];
    scores_file.read_to_string(&mut scores_string).unwrap_or(0);

    // Skip if there's no content in the file.
    if !scores_string.is_empty() {
        scores = Score::parse_and_merge(scores_string, ARK_SHELF_WORKING_DIR.as_path());
    }

    dbg!(&scores);

    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    // Merge scores item and write to score file.
    scores_file
        .write_all(Score::into_lines(&scores).as_bytes())
        .unwrap();

    let builder = tauri::Builder::default();
    let builder = set_command(builder);
    builder
        .manage(cli)
        .manage(Arc::new(Mutex::new(scores)))
        .setup(|app| {
            let handle = app.handle();
            let path = (*ARK_SHELF_WORKING_DIR).clone();
            init_link_watcher(path, handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
