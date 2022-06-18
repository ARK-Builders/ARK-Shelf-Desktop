use std::hash::{Hash, Hasher};
use std::{collections::hash_map::DefaultHasher, fmt};
use std::{fs::File, io::Write, path::PathBuf};

use scraper::{Html, Selector};
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
    /// Get metadata of the link.
    pub async fn get_preview(url: String) -> Result<OpenGraph, reqwest::Error> {
        let scraper = reqwest::get(url).await?.text().await?;
        let html = Html::parse_document(&scraper.as_str());

        Ok(OpenGraph {
            title: select_og(&html, OpenGraphTag::Title),
            description: select_og(&html, OpenGraphTag::Description),
            url: select_og(&html, OpenGraphTag::Url),
            image: select_og(&html, OpenGraphTag::Image),
            object_type: select_og(&html, OpenGraphTag::Type),
            locale: select_og(&html, OpenGraphTag::Locale),
        })
    }
}

fn select_og(html: &Html, tag: OpenGraphTag) -> Option<String> {
    let selector = Selector::parse(&format!("meta[property=\"og:{}\"]", tag.as_str())).unwrap();

    if let Some(element) = html.select(&selector).next() {
        if let Some(value) = element.value().attr("content") {
            return Some(value.to_string());
        }
    }

    None
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenGraph {
    /// Represents the "og:title" OpenGraph meta tag.
    ///
    /// The title of your object as it should appear within
    /// the graph, e.g., "The Rock".
    title: Option<String>,
    /// Represents the "og:description" OpenGraph meta tag
    description: Option<String>,
    /// Represents the "og:url" OpenGraph meta tag
    url: Option<String>,
    /// Represents the "og:image" OpenGraph meta tag
    image: Option<String>,
    /// Represents the "og:type" OpenGraph meta tag
    ///
    /// The type of your object, e.g., "video.movie". Depending on the type
    /// you specify, other properties may also be required.
    object_type: Option<String>,
    /// Represents the "og:locale" OpenGraph meta tag
    locale: Option<String>,
}

/// OpenGraphTag meta tags collection
pub enum OpenGraphTag {
    /// Represents the "og:title" OpenGraph meta tag.
    ///
    /// The title of your object as it should appear within
    /// the graph, e.g., "The Rock".
    Title,
    /// Represents the "og:url" OpenGraph meta tag
    Url,
    /// Represents the "og:image" OpenGraph meta tag
    Image,
    /// Represents the "og:type" OpenGraph meta tag
    ///
    /// The type of your object, e.g., "video.movie". Depending on the type
    /// you specify, other properties may also be required.
    Type,
    /// Represents the "og:description" OpenGraph meta tag
    Description,
    /// Represents the "og:locale" OpenGraph meta tag
    Locale,
    /// Represents the "og:image:height" OpenGraph meta tag
    ImageHeight,
    /// Represents the "og:image:width" OpenGraph meta tag
    ImageWidth,
    /// Represents the "og:site_name" OpenGraph meta tag
    SiteName,
}

impl fmt::Debug for OpenGraphTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl OpenGraphTag {
    fn as_str(&self) -> &str {
        match self {
            OpenGraphTag::Title => "title",
            OpenGraphTag::Url => "url",
            OpenGraphTag::Image => "image",
            OpenGraphTag::Type => "type",
            OpenGraphTag::Description => "description",
            OpenGraphTag::Locale => "locale",
            OpenGraphTag::ImageHeight => "image:height",
            OpenGraphTag::ImageWidth => "image:width",
            OpenGraphTag::SiteName => "site_name",
        }
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
