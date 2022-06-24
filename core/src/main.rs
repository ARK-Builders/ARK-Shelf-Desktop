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
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
    time::Duration,
};
use walkdir::{DirEntry, WalkDir};

lazy_static! {
    pub static ref ARK_SHELF_DATA_PATH: PathBuf =
        PathBuf::from(Cli::parse().path).join(".ark").join("shelf");
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
        default_value_t = format!("{}/ark-shelf",home_dir().unwrap().display())
    )]
    path: String,
}

// Initialize file watcher.
fn init_score_watcher(path: String) {
    thread::spawn(|| {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher
            .watch(path, notify::RecursiveMode::NonRecursive)
            .unwrap();
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    // Append new link into score file
                    DebouncedEvent::Create(path) => {
                        let score = Score {
                            hash: Score::calc_hash(path),
                            value: 0,
                        };
                        let mut score_file = File::options()
                            .append(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        score_file
                            .write_all(format!("\n{}", score.to_string()).as_bytes())
                            .unwrap();
                    }
                    // Remove from score file
                    DebouncedEvent::NoticeRemove(_) => {
                        // let score = Score {
                        //     hash: Score::calc_hash(path),
                        //     value: 0,
                        // };
                        let mut buf = String::new();
                        let mut scores_file = File::options()
                            .read(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        scores_file.read_to_string(&mut buf).unwrap();

                        let scores = Score::parse(buf);
                        let merged = merge_scores(Cli::parse().path, scores);
                        let mut merged_scores_file = File::options()
                            .write(true)
                            .truncate(true)
                            .open(SCORES_PATH.as_path())
                            .unwrap();
                        merged_scores_file
                            .write_all(Score::into_lines(merged).as_bytes())
                            .unwrap();
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

// Merge scores with provided score list.
fn merge_scores(path: impl AsRef<Path>, merge_scores: Scores) -> Scores {
    let entrys = WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_string()
                .ends_with(".link")
        })
        .map(|e| e.unwrap())
        .collect::<Vec<DirEntry>>();

    let init_scores = entrys
        .iter()
        .map(|entry| Score {
            hash: Score::calc_hash(entry.path()),
            // Default to 0
            value: 0,
        })
        .collect::<Scores>();

    init_scores
        .iter()
        .map(|score|
            // Merge score item if the item already existed in to-be-merged scores
            // Item not found in init_scores will be ignored. (Remove from the list)
            match merge_scores.iter().find(|&s| s.hash == score.hash) {
            Some(item) => item.clone(),
            None => score.clone(),
        })
        .collect::<Scores>()
    // Score::into_lines(merged)
}

fn main() {
    let cli = Cli::parse();
    lazy_static::initialize(&SCORES_PATH);
    lazy_static::initialize(&ARK_SHELF_DATA_PATH);
    std::fs::create_dir_all(&cli.path).unwrap();
    std::fs::create_dir_all(ARK_SHELF_DATA_PATH.as_path()).unwrap();
    // Check if the scores file is existed, otherwise create one.
    let mut scores_file = File::options()
        .read(true)
        .open(SCORES_PATH.as_path())
        .unwrap_or_else(|_| File::create(SCORES_PATH.as_path()).unwrap());

    let mut scores_string = String::new();

    scores_file.read_to_string(&mut scores_string).unwrap_or(0);
    let mut prepare_merge = vec![];
    // Skip if there's no content in the file.
    if !scores_string.is_empty() {
        prepare_merge = Score::parse(scores_string);
    }
    dbg!(&prepare_merge);
    let merged = merge_scores(&cli.path, prepare_merge);

    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    // Merge scores item and write to score file.
    scores_file
        .write_all(Score::into_lines(merged).as_bytes())
        .unwrap();

    init_score_watcher(cli.path.clone());

    let builder = tauri::Builder::default();
    let builder = set_command(builder);
    builder
        .manage(cli)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
