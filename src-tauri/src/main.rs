#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod base;
mod cli;
mod command;
use crate::cli::*;
use base::Score;
use base::Scores;
use command::errors::{CommandError, Result};
use command::subcommand::process_subcommand;
use command::subcommand::{self, add, set_command};

use log::info;
use log::LevelFilter;
use notify_debouncer_full::{
    new_debouncer,
    notify::{EventKind, RecursiveMode, Watcher},
};
use std::path;
use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, SystemTime},
};
use tauri::{AppHandle, Manager};
use tauri_plugin_log::fern::colors::Color;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::LogTarget;

static ARK_SHELF_WORKING_DIR: OnceLock<PathBuf> = OnceLock::new();
static SCORES_PATH: OnceLock<PathBuf> = OnceLock::new();
static METADATA_PATH: OnceLock<PathBuf> = OnceLock::new();
static PREVIEWS_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GraphMetaData {
    description: Option<String>,
    title: Option<String>,
    image_url: Option<String>,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PreviewLoaded {
    url: String,
    graph: GraphMetaData,
    created_time: Option<SystemTime>,
}

fn init_statics(path: PathBuf) {
    let scores_path = path.join(arklib::ARK_FOLDER).join("scores");
    SCORES_PATH.set(scores_path).unwrap();
    let metadata_folder = path
        .join(arklib::ARK_FOLDER)
        .join(arklib::METADATA_STORAGE_FOLDER);
    METADATA_PATH.set(metadata_folder).unwrap();
    let preview_folder = path
        .join(arklib::ARK_FOLDER)
        .join(arklib::PREVIEWS_STORAGE_FOLDER);
    PREVIEWS_PATH.set(preview_folder).unwrap();
}

pub fn init_dirs() {
    std::fs::create_dir_all(ARK_SHELF_WORKING_DIR.get().unwrap().join(arklib::ARK_FOLDER)).unwrap();
    std::fs::create_dir_all(METADATA_PATH.get().unwrap()).unwrap();
    std::fs::create_dir_all(PREVIEWS_PATH.get().unwrap()).unwrap();

    let scores_path = SCORES_PATH.get().unwrap();
    if let Err(_) = std::fs::metadata(SCORES_PATH.get().unwrap()) {
        File::create(scores_path).unwrap();
    }
}

pub fn init_scores(app: &tauri::App) {
    let scores_mutex: tauri::State<'_, Arc<Mutex<Scores>>> = app.state();
    let scores_path = SCORES_PATH.get().unwrap();
    let scores_string = std::fs::read_to_string(scores_path).unwrap();
    let mut scores = scores_mutex.lock().unwrap();

    // Skip if there's no content in the file.
    if !scores_string.is_empty() {
        let mut score: Scores =
            Score::parse_and_merge(scores_string, ARK_SHELF_WORKING_DIR.get().unwrap().join(arklib::ARK_FOLDER));
        scores.append(&mut score)
    }

    dbg!(&scores);
}

async fn get_preview(path: &PathBuf, manager: AppHandle) -> Result<()> {
    let file_content = std::fs::read_to_string(path)?;
    let url = url::Url::parse(&file_content)?;
    let id = arklib::id::ResourceId::compute_bytes(&url.as_str().as_bytes())
        .map_err(|_| CommandError::Arklib)?;
    let graph_preview = arklib::link::Link::get_preview(url.to_string())
        .await
        .map_err(|_| CommandError::Arklib)?;
    let image_data = graph_preview
        .fetch_image()
        .await
        .ok_or(CommandError::Arklib)?;
    let preview_folder = PREVIEWS_PATH.get().unwrap();
    std::fs::write(preview_folder.join(format!("{id}")), image_data)?;

    let mut created_time = None;
    if let Ok(meta) = std::fs::metadata(path) {
        created_time = meta.created().ok();
    }
    let graph_data = GraphMetaData {
        image_url: graph_preview.image,
        title: graph_preview.title,
        description: graph_preview.description,
    };
    let preview_loaded = PreviewLoaded {
        url: url.into(),
        graph: graph_data,
        created_time,
    };
    manager.emit_all("preview_loaded", preview_loaded).unwrap();
    Ok(())
}

fn init_link_watcher(path: PathBuf, app_handle: AppHandle) {
    let path = path.clone();
    let handle = app_handle.clone();
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
        let score_path = SCORES_PATH.get().unwrap();
        loop {
            match rx.recv() {
                Ok(Ok(events)) => {
                    events.into_iter().for_each(|event| {
                        if let EventKind::Create(_) = event.kind {
                            event.event.paths.into_iter().for_each(|path| {
                                if &path != score_path {
                                    let manager = handle.clone();
                                    tauri::async_runtime::spawn(async move {
                                        let _ = get_preview(&path, manager).await;
                                    });
                                }
                            });
                        }
                    });
                }
                Ok(Err(e)) => {
                    eprintln!("Errors on with the notifier watcher: {e:?}");
                }
                Err(_) => {
                    eprintln!("Error with Watcher channel!");
                    break;
                }
            }
        }
    });
}

fn cli_unknown_arg(key: String, app: tauri::AppHandle) {
    println!("sleeping for unhandled cli arg: {}", key);
    std::thread::sleep(std::time::Duration::from_secs(5));
    app.exit(1);
}

fn run_gui(app: tauri::App, path_buf: PathBuf) {
    let handle = app.handle();
    init_statics(path_buf.clone());
    init_dirs();
    init_scores(&app);
    init_link_watcher(path_buf.clone(), handle);
    app.run(move |_, event| {
        match event {
            _ => {}
        }
    });
}

fn main() {
    let scores: Vec<Score> = Vec::new();
    let builder = tauri::Builder::default().plugin(
        tauri_plugin_log::Builder::default()
            .targets([LogTarget::Stdout, LogTarget::Webview])
            .with_colors(ColoredLevelConfig::default())
            .level(LevelFilter::Info)
            .build(),
    );
    let builder = set_command(builder);
    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");
    let handle = app.handle();
    handle.manage(Arc::new(Mutex::new(scores)));
    let matches = app.get_cli_matches().unwrap();

    if let Some(sub) = matches.subcommand {
        let path_buf = std::env::current_dir().unwrap();
        ARK_SHELF_WORKING_DIR.set(path_buf.clone()).unwrap();

        init_statics(path_buf.clone());
        init_dirs();
        process_subcommand(sub, &app);
    }
    
    println!("args len: {:?}", matches.args);
    if let Some(_) = matches.args.get_key_value("help") {
        println!("This is help option");
        std::process::exit(1);
    }

    let (_, add_value) = matches.args.get_key_value("add").unwrap(); 
    if add_value.occurrences.gt(&0) {
        let path = ARK_SHELF_WORKING_DIR.get_or_init(|| std::env::current_dir().unwrap());
        let manager = handle.clone();
        tauri::async_runtime::spawn(async move {
            let _ = get_preview(&path, manager).await;
        });
    }

    let (_, path_value) = matches.args.get_key_value("path").unwrap();
    if  path_value.occurrences.gt(&0) {
        let path_buf = match path_value.value.as_str() {
            Some(str) => PathBuf::from_str(str).unwrap(),
            None => std::env::current_dir().unwrap()
        };
        info!("ARK SHELF working directory updated to {:?}\n", path_buf);
        ARK_SHELF_WORKING_DIR.set(path_buf.clone()).unwrap();
        run_gui(app, path_buf);
    } else {
        let path_buf = std::env::current_dir().unwrap();
        ARK_SHELF_WORKING_DIR.set(path_buf.clone()).unwrap();
        info!("ARK SHELF working directory updated to {:?}\n", path_buf);
        
        run_gui(app, path_buf);
    }
}
