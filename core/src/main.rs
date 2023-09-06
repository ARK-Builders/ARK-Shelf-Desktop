#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod base;
mod command;
use base::{Score, Scores};
use clap::Parser;
use command::*;
use home::home_dir;
use lazy_static::lazy_static;
use notify::{watcher, DebouncedEvent, Watcher};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::Manager;

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

// Initialize file watcher.
fn init_score_watcher(path: String, scores: Arc<Mutex<Scores>>) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_millis(300)).unwrap();
        watcher
            .watch(path, notify::RecursiveMode::NonRecursive)
            .unwrap();
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    // Append new link into score file
                    DebouncedEvent::Create(path) => {
                        let score = Score {
                            name: path.file_name().unwrap().to_string_lossy().to_string(),
                            hash: Score::calc_hash(path).expect("Error computing hash"),
                            value: 0,
                        };
                        scores.lock().unwrap().push(score.clone());
                        dbg!(&scores);
                        let mut score_file = File::options()
                            .write(true)
                            .append(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        writeln!(score_file, "{}", score.to_string()).unwrap();
                    }
                    // Remove from score file
                    DebouncedEvent::NoticeRemove(path) => {
                        let removed_link_name =
                            path.file_name().unwrap().to_string_lossy().to_string();
                        dbg!(&removed_link_name);
                        let filtered_scores = scores
                            .lock()
                            .unwrap()
                            .iter()
                            .filter(|s| s.name != removed_link_name)
                            .map(|s| s.clone())
                            .collect::<Vec<_>>();

                        *scores.lock().unwrap() = filtered_scores.clone();
                        dbg!(&scores);

                        let mut buf = String::new();
                        let mut scores_file = File::options()
                            .read(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        scores_file.read_to_string(&mut buf).unwrap();

                        let mut merged_scores_file = File::options()
                            .write(true)
                            .truncate(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        if !filtered_scores.is_empty() {
                            merged_scores_file
                                .write_all(Score::into_lines(filtered_scores).as_bytes())
                                .unwrap();
                        }
                    }
                    _ => {}
                },
                Err(e) => {
                    panic!("{e}")
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
    } else {
        scores = Score::merge(scores, ARK_SHELF_WORKING_DIR.as_path());
    }

    dbg!(&scores);

    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    // Merge scores item and write to score file.
    scores_file
        .write_all(Score::into_lines(scores.clone()).as_bytes())
        .unwrap();

    let builder = tauri::Builder::default();
    let builder = set_command(builder);
    builder
        .manage(cli)
        .manage(Arc::new(Mutex::new(scores)))
        .setup(|app| {
            let state_scores = app.state::<Arc<Mutex<Scores>>>();
            init_score_watcher(
                ARK_SHELF_WORKING_DIR.to_str().unwrap().to_string(),
                state_scores.inner().clone(),
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
