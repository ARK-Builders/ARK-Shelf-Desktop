use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use serde::{Serialize};

use crate::{
    base::{Link, OpenGraph, Score, Scores},
    Cli, ARK_SHELF_WORKING_DIR, SCORES_PATH,
};

use tauri::{Builder, Runtime};
use url::Url;
use walkdir::{DirEntry, WalkDir};

#[tauri::command(async)]
/// Create a `.link`
async fn create_link(
    title: String,
    desc: String,
    url: String,
    state: tauri::State<'_, Cli>,
) -> Result<(), String> {
    let url = match Url::parse(url.as_str()) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    let mut link = arklib::link::Link::new(title, desc, url);
    let name = format!("{}.link", link.format_name());
    dbg!(&name);
    link.write_to_path(PathBuf::from(format!("{}/{}", &state.path, name)), true)
        .await;
    sleep(Duration::from_millis(305));
    Ok(())
}


#[tauri::command(async)]
/// Remove a `.link` from directory
fn delete_link(name: String, state: tauri::State<Cli>) {
    fs::remove_file(format!("{}/{}", &state.path, name)).expect("cannot remove the link");
    sleep(Duration::from_millis(305));
}

fn get_fs_links() -> Vec<DirEntry> {
    WalkDir::new(ARK_SHELF_WORKING_DIR.as_path())
        .max_depth(1)
        .into_iter()
        .filter(|file| {
            file.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .to_string()
                .ends_with(".link")
        })
        .map(|e| e.unwrap())
        .collect::<Vec<DirEntry>>()
}

#[tauri::command(async)]
/// Read names of `.link` in user specific directory
fn read_link_list() -> Vec<String> {
    let mut path_list = vec![];
    for item in get_fs_links() {
        dbg!(&item);
        let file_name = item.file_name().to_str().unwrap().to_string();
        path_list.push(file_name);
    }
    dbg!(&path_list);
    path_list
}

#[tauri::command(async)]
async fn generate_link_preview(url: String) -> Result<OpenGraph, String> {
    Link::get_preview(url).await.map_err(|e| e.to_string())
}

/// Get the score list
#[tauri::command(async)]
fn get_scores(scores: tauri::State<Arc<Mutex<Scores>>>) -> Result<Scores, String> {
    Ok(scores.lock().unwrap().clone())
}

/// Set scores
///
/// Only affected scores file.
#[tauri::command(async)]
fn set_scores(
    scores: Scores,
    state_scores: tauri::State<Arc<Mutex<Scores>>>,
) -> Result<(), String> {
    *state_scores.lock().unwrap() = scores.clone();
    dbg!(&scores);
    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    scores_file
        .write_all(Score::into_lines(scores).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Wrapper around the arklib::link::Link struct.
#[derive(Debug, Serialize)]
pub struct LinkWrapper{
    title: String,
    desc: String,
    url: Url,
    
    // Only shared on desktop
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<std::time::SystemTime>,
}

#[tauri::command(async)]
/// Read data from `.link` file
fn read_link(name: String, state: tauri::State<Cli>) -> LinkWrapper {
    let file_path = PathBuf::from(format!("{}/{}", &state.path, name));
    let link = Link::from(file_path.to_owned());
    let file = File::open(file_path.to_owned()).unwrap();
    let created_time = file.metadata().unwrap().created().unwrap();
    // dbg!(&link);
    LinkWrapper {
        created_time: Some(created_time),
        title: link.title,
        desc: link.desc,
        url: link.url,
    }
}

pub fn set_command<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        create_link,
        read_link_list,
        delete_link,
        generate_link_preview,
        read_link,
        get_scores,
        set_scores
    ])
}
