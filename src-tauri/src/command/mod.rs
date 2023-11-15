use serde::Serialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod errors;
pub use errors::{CommandError, Result};

use crate::{
    base::{Link, OpenGraph, Score, Scores},
    Cli, ARK_SHELF_WORKING_DIR, METADATA_PATH, PREVIEWS_PATH, SCORES_PATH,
};

use tauri::{Builder, Runtime};
use url::Url;
use walkdir::{DirEntry, WalkDir};

#[derive(serde::Serialize)]
pub struct LinkScoreMap {
    pub name: String,
    pub hash: String,
    pub value: i64
}

#[tauri::command]
/// Create a `.link`
async fn create_link(
    url: String,
    state: tauri::State<'_, Cli> , 
    metadata: arklib::link::Metadata
) -> Result<()> {
    super::create_link(&url, &state.path, metadata).expect("Error creating link");
    Ok(())
}

#[tauri::command]
/// Remove a `.link` from directory
async fn delete_link(name: String) -> Result<()> {
    let path = ARK_SHELF_WORKING_DIR.get().unwrap().join(&name);
    let content = fs::read_to_string(&path)?;
    let id = arklib::id::ResourceId::compute_bytes(content.as_bytes())
        .map_err(|_| CommandError::Arklib)?;
    fs::remove_file(&path)?;

    let id = id.to_string();
    let metadata_path = METADATA_PATH.get().unwrap().join(&id);
    let preview_path = PREVIEWS_PATH.get().unwrap().join(&id);
    fs::remove_file(metadata_path).ok();
    fs::remove_file(preview_path).ok();
    Ok(())
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
async fn generate_link_preview(url: String) -> Result<OpenGraph> {
    Link::get_preview(url).await.map_err(|_| CommandError::IO)
}

/// Get the score list
#[tauri::command]
async fn get_scores(scores: tauri::State<'_, Arc<Mutex<Scores>>>) -> Result<Scores> {
    let scores_content = std::fs::read(SCORES_PATH.get().unwrap())?;
    let scores_content = String::from_utf8(scores_content)?;
    let scores_files = Score::parse_and_merge(scores_content, ARK_SHELF_WORKING_DIR.get().unwrap());
    let mut guard = scores.lock().unwrap();
    *guard = scores_files.clone();
    Ok(scores_files)
}

#[tauri::command]
async fn add(scores: tauri::State<'_, Arc<Mutex<Scores>>>, name: String) -> Result<Option<Score>> {
    let mut guard = scores.lock().unwrap();
    let mut result = None;
    if let Some(score) = guard.iter_mut().find(|score| score.name == name) {
        score.value += 1;
        result = Some(score.clone());
        let content = Score::into_lines(&*guard);
        std::fs::write(SCORES_PATH.get().unwrap(), content).unwrap();
    }
    Ok(result)
}

#[tauri::command]
async fn substract(
    scores: tauri::State<'_, Arc<Mutex<Scores>>>,
    name: String,
) -> Result<Option<Score>> {
    let mut guard = scores.lock().unwrap();
    let mut result = None;
    if let Some(score) = guard.iter_mut().find(|score| score.name == name) {
        score.value -= 1;
        result = Some(score.clone());
        let content = Score::into_lines(&*guard);
        std::fs::write(SCORES_PATH.get().unwrap(), content).unwrap();
    }
    Ok(result)
}

#[tauri::command]
async fn create_score(
    scores: tauri::State<'_, Arc<Mutex<Scores>>>,
    url: String,
    value: i64,
) -> Result<Score> {
    let mut score = Score::new(&url);
    score.value = value;
    let mut guard = scores.lock().unwrap();
    guard.push(score.clone());
    let content = Score::into_lines(&*guard);
    std::fs::write(SCORES_PATH.get().unwrap(), content)?;
    Ok(score)
}

/// Set scores
///
/// Only affected scores file.
#[tauri::command]
async fn set_scores(
    scores: Scores,
    state_scores: tauri::State<'_, Arc<Mutex<Scores>>>,
) -> Result<()> {
    let guard = state_scores.lock().unwrap();
    dbg!(&scores);
    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.get().unwrap())?;
    scores_file.write_all(Score::into_lines(&*guard).as_bytes())?;
    Ok(())
}

/// Wrapper around the arklib::link::Link struct.
#[derive(Debug, Serialize)]
pub struct LinkWrapper {
    title: String,
    desc: Option<String>,
    url: Url,

    /// Only used on desktop, not in mobile version
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<std::time::SystemTime>,
}

#[tauri::command]
/// Read data from `.link` file
async fn read_link(name: String) -> Result<LinkWrapper> {
    let file_path = PathBuf::from(ARK_SHELF_WORKING_DIR.get().unwrap()).join(&name);
    let link = Link::load(ARK_SHELF_WORKING_DIR.get().unwrap(), &file_path)
        .map_err(|_| CommandError::Arklib)?;
    let meta = fs::metadata(&file_path)?;
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

fn get_fs_links() -> Vec<DirEntry> {
    WalkDir::new(ARK_SHELF_WORKING_DIR.get().unwrap())
        .max_depth(1)
        .into_iter()
        .filter(|file| {
            file.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with(".link")
        })
        .map(|e| e.unwrap())
        .collect::<Vec<DirEntry>>()
}

pub fn set_command<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        create_link,
        read_link_list,
        delete_link,
        generate_link_preview,
        read_link,
        get_scores,
        set_scores,
        add,
        substract,
        create_score,
    ])
}
