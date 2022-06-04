use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;
/// .link File used in ARK Shelf.
#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    title: String,
    desc: String,
    url: Url,
}

impl Link {
    pub fn new(title: String, desc: String, url: Url) -> Self {
        Self { title, desc, url }
    }
    /// Get formatted name for .link
    pub fn format_name(&self) -> String {
        self.url
            .to_string()
            .replace("http://", "")
            .replace("https://", "")
            .split(&['-', '?', '/'][..])
            .filter(|x| x != &"")
            .collect::<Vec<&str>>()
            .join("-")
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
        let mut zip = zip::ZipArchive::new(file).unwrap();
        let j_raw = zip.by_name("link.json").unwrap();
        serde_json::from_reader(j_raw).unwrap()
    }
}
