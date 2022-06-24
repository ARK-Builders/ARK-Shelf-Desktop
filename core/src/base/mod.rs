mod link;
use std::{fs::File, path::Path};

pub use link::{Link, OpenGraph};
use serde::{Deserialize, Serialize};

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
    pub hash: String,
    // Score could take a negative value.
    pub value: i64,
}

impl Score {
    pub fn calc_hash(path: impl AsRef<Path>) -> String {
        format!(
            "{:x}",
            arklib::id::ResourceId::compute(
                File::open(&path).unwrap().metadata().unwrap().len(),
                path,
            )
            .crc32
        )
    }
    pub fn parse(content: String) -> Scores {
        let splited = content
            .split("\n")
            .map(|val| {
                let mapped = val.split(": ").collect::<Vec<&str>>();
                Score {
                    hash: mapped[0].to_string(),
                    value: i64::from_str_radix(mapped[1], 10).unwrap(),
                }
            })
            .collect::<Vec<Score>>();
        splited
    }

    pub fn format(hash: String, value: i64) -> String {
        String::from(format!("{hash}: {value}"))
    }
    pub fn into_lines(arr: Scores) -> String {
        arr.iter()
            .map(|s| Score::format(s.hash.clone(), s.value))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl ToString for Score {
    fn to_string(&self) -> String {
        Score::format(self.hash.clone(), self.value)
    }
}
