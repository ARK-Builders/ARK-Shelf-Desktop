use serde::Serialize;
use std::{
    fs::{self, File},
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    base::{Link, OpenGraph, Score, Scores},
    Cli, ARK_SHELF_WORKING_DIR, SCORES_PATH,
};

use tauri::{Builder, Runtime};
use url::Url;
use walkdir::{DirEntry, WalkDir};

#[tauri::command]
/// Create a `.link`
async fn create_link(
    title: String,
    desc: Option<String>,
    url: String,
    state: tauri::State<'_, Cli>,
) -> Result<(), String> {
    let url = match Url::parse(url.as_str()) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    let ressource = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute ressource from url");
    let domain = url.domain().expect("Url has no domain");
    let path = format!("{}/{domain}-{}.link", &state.path, ressource.crc32);
    let mut link = Link::new(url, title, desc);
    link.write_to_path(&state.path, &path, true)
        .await
        .expect("Custom error type needed");
    Ok(())
}

#[tauri::command]
/// Remove a `.link` from directory
async fn delete_link(name: String, state: tauri::State<'_, Cli>) -> Result<(), ()> {
    fs::remove_file(format!("{}/{}", &state.path, name)).expect("cannot remove the link");
    Ok(())
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

#[tauri::command]
/// Read names of `.link` in user specific directory
async fn read_link_list() -> Vec<String> {
    let mut path_list = vec![];
    for item in get_fs_links() {
        dbg!(&item);
        let file_name = item.file_name().to_str().unwrap().to_string();
        path_list.push(file_name);
    }
    dbg!(&path_list);
    path_list
}

#[tauri::command]
async fn generate_link_preview(url: String) -> Result<OpenGraph, String> {
    Link::get_preview(url).await.map_err(|e| e.to_string())
}

/// Get the score list
#[tauri::command]
async fn get_scores(scores: tauri::State<'_, Arc<Mutex<Scores>>>) -> Result<Scores, String> {
    Ok(scores.lock().unwrap().clone())
}

/// Set scores
///
/// Only affected scores file.
#[tauri::command]
async fn set_scores(
    scores: Scores,
    state_scores: tauri::State<'_, Arc<Mutex<Scores>>>,
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
pub struct LinkWrapper {
    title: String,
    desc: Option<String>,
    url: Url,
    // // Only shared on desktop
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<std::time::SystemTime>,
}

#[tauri::command]
/// Read data from `.link` file
async fn read_link(name: String, state: tauri::State<'_, Cli>) -> Result<LinkWrapper, ()> {
    let file_path = format!("{}/{name}", &state.path);
    let link = Link::load(&state.path, &file_path).expect(&format!("Error loading {file_path}"));
    let meta = fs::metadata(&file_path).expect("Error loading metadata");
    let created_time = match meta.created() {
        Ok(time) => Some(time),
        Err(_) => None,
    };
    Ok(LinkWrapper {
        title: link.meta.title,
        desc: link.meta.desc,
        url: link.url,
        created_time,
    })
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
