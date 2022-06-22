mod link;
pub use link::{Link, OpenGraph};
use serde::{Deserialize, Serialize};
/// ARK Config
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Sorting mode.
    pub mode: Mode,
    /// The score content, repensented to a score.
    pub score: Option<Vec<String>>,
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
