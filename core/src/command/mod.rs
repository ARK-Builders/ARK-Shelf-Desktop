use std::{
    ffi::OsString,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{
    base::{Link, LinkScoreMap, OpenGraph, Score},
    Cli, SCORES_PATH,
};

use tauri::{Builder, Runtime};
use url::Url;
use walkdir::{DirEntry, WalkDir};

#[tauri::command]
/// Create a `.lin&mut k`
fn create_link(
    title: String,
    desc: String,
    url: String,
    state: tauri::State<Cli>,
) -> Result<(), String> {
    let url = match Url::parse(url.as_str()) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    let link = Link::new(title, desc, url);
    let name = format!("{}.link", link.format_name());
    println!("{}", name);
    link.write_to_path(PathBuf::from(format!("{}/{}", &state.path, name)));
    Ok(())
}

#[tauri::command(async)]
/// Remove a `.link` from directory
fn delete_link(name: String, state: tauri::State<Cli>) {
    fs::remove_file(format!("{}/{}", &state.path, name)).expect("cannot remove the link");
}

fn get_fs_links(state: tauri::State<Cli>) -> Vec<DirEntry> {
    WalkDir::new(state.path.clone())
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
fn read_link_list(state: tauri::State<Cli>) -> Vec<String> {
    let mut path_list = vec![];
    for item in get_fs_links(state.clone()) {
        let file_name = item.file_name().to_str().unwrap().to_string();
        path_list.push(file_name);
    }
    path_list
}

#[tauri::command(async)]
async fn generate_link_preview(url: String) -> Result<OpenGraph, String> {
    Link::get_preview(url).await.map_err(|e| e.to_string())
}

/// Get the score list
#[tauri::command(async)]
fn get_scores(state: tauri::State<Cli>) -> Result<Vec<LinkScoreMap>, String> {
    let mut file = File::open(SCORES_PATH.as_path()).map_err(|e| e.to_string())?;
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf)
        .map_err(|e| e.to_string())?;
    let scores = Score::parse(string_buf);
    let fs_links = get_fs_links(state);

    let link_score_maps = fs_links
        .iter()
        .map(|entry| {
            let item = scores
                .iter()
                .find(|s| Score::calc_hash(entry.path()) == s.hash)
                .unwrap();
            LinkScoreMap {
                name: entry.file_name().to_str().unwrap().to_string(),
                value: item.value,
            }
        })
        .collect::<Vec<_>>();
    Ok(link_score_maps)
}

/// Set scores
///
/// Only affected scores file.
#[tauri::command(async)]
fn set_scores(link_score_maps: Vec<LinkScoreMap>, state: tauri::State<Cli>) -> Result<(), String> {
    let mut buf = String::new();
    let mut scores_file = File::options()
        .read(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    scores_file
        .read_to_string(&mut buf)
        .map_err(|e| e.to_string())?;

    let scores = Score::parse(buf);
    let fs_links = get_fs_links(state);

    let transformed = link_score_maps
        .iter()
        .map(|s| {
            let item = fs_links
                .iter()
                .find(|e| e.file_name().to_os_string() == OsString::from(s.name.clone()))
                .unwrap();

            Score {
                hash: Score::calc_hash(item.path()),
                value: s.value,
            }
        })
        .collect::<Vec<_>>();

    let merged = scores
        .iter()
        .map(|s| match transformed.iter().find(|&ms| ms.hash == s.hash) {
            Some(item) => item.clone(),
            None => s.clone(),
        })
        .collect::<Vec<_>>();

    let mut scores_file = File::options()
        .write(true)
        .truncate(true)
        .open(SCORES_PATH.as_path())
        .unwrap();
    scores_file
        .write_all(Score::into_lines(merged).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command(async)]
/// Read data from `.link` file
fn read_link(name: String, state: tauri::State<Cli>) -> Link {
    let link = Link::from(PathBuf::from(format!("{}/{}", &state.path, name)));
    // dbg!(&link);
    return link;
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
