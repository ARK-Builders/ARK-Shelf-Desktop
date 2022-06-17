use std::{fs, path::PathBuf};

use crate::{base::Link, Cli};

use tauri::{Builder, Runtime};
use url::{ParseError, Url};
use walkdir::WalkDir;

#[tauri::command]
/// Create a `.link`
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

#[tauri::command(async)]
/// Read names of `.link` in user specific directory
fn read_link_list(state: tauri::State<Cli>) -> Vec<String> {
    let mut path_list = vec![];
    for item in WalkDir::new(state.path.clone()).max_depth(1).into_iter() {
        let file_name = item.unwrap().file_name().to_str().unwrap().to_string();
        if file_name.ends_with(".link") {
            path_list.push(file_name);
        }
    }
    path_list
}

#[tauri::command(async)]
/// Read data from `.link` file
fn read_link(name: String, state: tauri::State<Cli>) -> Link {
    let link = Link::from(PathBuf::from(format!("{}/{}", &state.path, name)));
    println!("{:?}", link);
    return link;
}

pub fn set_command<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        create_link,
        read_link,
        read_link_list,
        delete_link
    ])
}
