use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;
/// .link File used in ARK Shelf.
#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    title: String,
    desc: String,
    url: Url,

    created_time: Option<std::time::SystemTime>,
}

impl Link {
    pub fn new(title: String, desc: String, url: Url) -> Self {
        let created_time = std::time::SystemTime::now();
        Self {
            title,
            desc,
            url,
            created_time: Some(created_time),
        }
    }
    /// Get formatted name for .link
    pub fn format_name(&self) -> String {
        let mut s = DefaultHasher::new();

        let url = self
            .url
            .to_string()
            .replace("http://", "")
            .replace("https://", "")
            .split(&['-', '?', '/'][..])
            .filter(|x| x != &"")
            .collect::<Vec<&str>>()
            .join("-");
        url.hash(&mut s);
        s.finish().to_string()
    }
    /// Write zipped file to path
    pub fn write_to_path(&self, path: PathBuf) {
        let j = serde_json::to_string(self).unwrap();
        let link_file = File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(link_file);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file("link.json", options)
            .expect("cannot create link.json");
        zip.write(j.as_bytes()).unwrap();
        zip.finish().unwrap();
    }
}

impl From<PathBuf> for Link {
    fn from(path: PathBuf) -> Self {
        let file = File::open(path).unwrap();
        let created_time = file.metadata().unwrap().created().unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let j_raw = zip.by_name("link.json").unwrap();

        let j = serde_json::from_reader(j_raw).unwrap();
        Self {
            created_time: Some(created_time),
            ..j
        }
    }
}
