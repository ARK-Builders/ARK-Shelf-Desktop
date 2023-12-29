use std::path::PathBuf;

use arklib::link::Metadata;
use url::Url;

use crate::{ARK_SHELF_WORKING_DIR, command::errors::{Result, CommandError}, METADATA_PATH};

#[derive(Default, Debug)]
pub struct Cli {
    pub path: String,
    pub link: Option<Link>,
}

impl Cli {
    pub fn add_new_link(&self) -> bool {
        if let Some(link) = &self.link {
                        match link {
                Link::Add(l) => {
                    let metadata = Metadata {
                        title: l.title.clone(),
                        desc: l.description.clone(),
                    };
                    create_link(&l.url, metadata).expect("Creating Link failed");
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Debug)]
pub enum Link {
    /// Adds a new link
    Add(AddLink),
}

#[derive(Default, Debug)]
pub struct AddLink {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

/// Creates a `.link`
pub fn create_link(
    url: &str,
    metadata: arklib::link::Metadata,
) -> Result<String> {
    let root_path = ARK_SHELF_WORKING_DIR.get().unwrap().join(arklib::ARK_FOLDER);
    let url = Url::parse(url).expect("Error parsing url");
    let id = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute resource from url");
    let domain = url.domain().expect("Url has no domain");
    let file_name = format!("{domain}-{id}.link");
    let path = root_path.join(file_name.clone());
    if std::fs::metadata(&path).is_ok() {
        Err(CommandError::LinkExist)
    } else {
        println!("path: {:?}", path);
        std::fs::write(path, url.as_str()).expect("writing file failed");

        let meta_path: PathBuf = METADATA_PATH.get().unwrap().join(format!("{id}"));
        std::fs::write(&meta_path, serde_json::to_string(&metadata).unwrap()).expect("writing metadata failed");
        Ok(file_name)
    }
}

#[cfg(test)]
mod test {
    use crate::{init_statics, init_dirs};

    use super::*;

    #[test]
    fn add_link() {
        let path_buf = std::env::current_dir().unwrap();
        ARK_SHELF_WORKING_DIR.set(path_buf.clone()).unwrap();
        init_statics(path_buf.clone());
        init_dirs();

        let mut cli = Cli::default();
        cli.path = path_buf.to_str().unwrap().to_string();


        cli.link = Some(Link::Add(AddLink {
            url: "http://google.com".into(),
            title: "test".into(),
            description: None,
        }));
        cli.add_new_link();
    }
}
