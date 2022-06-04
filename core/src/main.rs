#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod base;
mod command;
use clap::Parser;
use command::*;
use home::home_dir;
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

fn main() {
    let cli = Cli::parse();
    std::fs::create_dir_all(&cli.path);
    let builder = tauri::Builder::default();
    let builder = set_command(builder);
    builder
        .manage(cli)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
