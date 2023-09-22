use std::path::Path;

pub use arklib::link::{Link, Metadata, OpenGraph};
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkScoreMap {
    pub name: String,
    pub value: i64,
}

/// ARK Config
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Sorting mode.
    pub mode: Mode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Sorting by Alphabet
    Normal,
    /// Sorting by date
    Date,
    /// Sorting by score
    Score,
}
pub type Scores = Vec<Score>;

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct Score {
    pub name: String,

    pub id: String,

    // Score could take a negative value.
    pub value: i64,
}

impl Score {
    pub fn new(url: &str) -> Self {
        let id = arklib::id::ResourceId::compute_bytes(url.as_bytes()).unwrap();
        let id = format!("{}-{}", id.crc32, id.data_size);
        let name = format!("{}.link", id);

        Score {
            id,
            name,
            value: 0,
        }
    }

    pub fn calc_id(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        let file_metadata = std::fs::metadata(&path)?;
        let id =
            arklib::id::ResourceId::compute(file_metadata.len(), path).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Error computing RessourceId",
                )
            })?;
        Ok(format!("{}-{}", id.crc32, id.data_size))
    }
    /// Parse scores from string.
    ///
    /// Note that the name in each item is set to default `String::new()`, since we can't parse name from crc32.
    fn parse(content: String) -> Scores {
        let splited = content
            .split("\n")
            .filter(|val| !val.is_empty())
            .map(|val| {
                let mapped = val.split(": ").collect::<Vec<&str>>();

                dbg!(&mapped);
                Score {
                    name: String::new(),
                    id: mapped[0].to_string(),
                    value: i64::from_str_radix(mapped[1], 10).unwrap_or(0),
                }
            })
            .collect::<Vec<Score>>();
        splited
    }
    /// Parse the given string into scores and merge scores by reading all `.link` in the given path.
    ///
    /// Scores name will be filled with `.link` name during merging.
    pub fn parse_and_merge(content: String, path: impl AsRef<Path>) -> Scores {
        let splited = Score::parse(content);
        let merged_scores = Score::merge(splited, path);
        merged_scores
    }
    /// Merge scores with reading given path.
    ///
    /// Scores name will be filled with `.link` name during merging.
    fn merge(merge_scores: Scores, path: impl AsRef<Path>) -> Scores {
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
                name: entry.file_name().to_string_lossy().to_string(),
                id: Score::calc_id(entry.path()).expect("Error computing id"),
                // Default to 0
                value: 0,
            })
            .collect::<Scores>();

        let merged_scores = init_scores
            .iter()
            .map(|score|
            // Merge score item if the item already existed in to-be-merged scores
            // Item not found in init_scores will be ignored. (Remove from the list)
            match merge_scores.iter().find(|&s| s.id == score.id) {
            // replace name with file name
            Some(item) => Score { name: score.name.clone(), ..item.clone() },
            None => score.clone(),
        })
            .collect::<Scores>();
        merged_scores
    }
    pub fn format(id: &str, value: i64) -> String {
        if value == 0 {
            return String::from(format!("{id}: "));
        }
        String::from(format!("{id}: {value}"))
    }
    pub fn into_lines(arr: &[Score]) -> String {
        let mut lines = arr
            .iter()
            .map(|s| Score::format(&s.id, s.value))
            .collect::<Vec<String>>()
            .join("\n");
        lines.push_str("\n");
        lines
    }
}

impl ToString for Score {
    fn to_string(&self) -> String {
        Score::format(&self.id, self.value)
    }
}
