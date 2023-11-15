use arklib::link::Properties;
use url::Url;

use crate::{command::errors::Result, ARK_SHELF_FOLDER, ARK_SHELF_WORKING_DIR};

#[derive(Default, Debug)]
pub struct Cli {
    pub path: String,
    pub link: Option<Link>,
}

impl Cli {
    pub async fn add_new_link(&self) -> bool {
        if let Some(link) = &self.link {
            match link {
                Link::Add(l) => {
                    let properties = Properties {
                        title: l.title.clone(),
                        desc: l.description.clone(),
                    };
                    create_link(&l.url, properties)
                        .await
                        .expect("Creating Link failed");
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

pub fn process_help() {
    let help_message = r#"
    ARK Shelf Desktop Help

    USAGE:
        shelf [SUBCOMMAND]
    
    SUBCOMMANDS:
        help      Shows this message or the help of the given subcommand(s)
    
        link      Manage links
            add    Adds a new link
                --path          Path where the link will be added
                --url           URL of the link
                --title         Title of the link
                --description   Description of the link
    
    OPTIONS:
        --path    Specifies the path (used with certain commands)
        --add     Add link using temporary GUI        
    
    Use 'shelf [SUBCOMMAND] --help' for more information about a specific subcommand.
    "#;
    println!("{}", help_message);
}

pub async fn create_link(url: &str, link: arklib::link::Properties) -> Result<String> {
    let root = ARK_SHELF_WORKING_DIR.get_or_init(|| {
        dirs::home_dir()
            .map(|home| home.join(ARK_SHELF_FOLDER))
            .unwrap()
    });
    let url = Url::parse(url).expect("Error parsing url");
    let id = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute resource from url");
    let domain = url.domain().expect("Url has no domain");
    let file_name = format!("{domain}-{id}.link");

    // let path = root.join(file_name.clone());
    let link: arklib::link::Link = arklib::link::Link { url, prop: link };

    link.save(root, true).await.unwrap();

    return Ok(file_name);
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use dirs::home_dir;

    #[test]
    fn add_link() {
        let temp_dir = home_dir().unwrap().join("tmp");
        if fs::metadata(temp_dir.clone()).is_ok() {
            fs::remove_dir_all(temp_dir.clone()).unwrap();
        }
        ARK_SHELF_WORKING_DIR.set(temp_dir.clone()).unwrap();
        crate::init_statics(temp_dir.clone());
        crate::init_dirs();
        let mut cli = Cli::default();
        cli.path = format!(
            "{}/.ark/tmp",
            home_dir().expect("Can't find home dir").display()
        );
        cli.link = Some(Link::Add(AddLink {
            url: "http://example.com".into(),
            title: "test".into(),
            description: None,
        }));

        tauri::async_runtime::block_on(async {
            cli.add_new_link().await;
        });
    }
}
