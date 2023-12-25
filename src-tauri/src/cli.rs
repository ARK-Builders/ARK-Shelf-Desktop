use arklib::link::Metadata;
use home::home_dir;
use url::Url;

use crate::ARK_SHELF_WORKING_DIR;

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
                    create_link(&l.url, metadata).expect("Creating Link");
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
) -> Result<(), String> {
    let root_path = ARK_SHELF_WORKING_DIR.get().and_then(|path| path.to_str()).unwrap();
    let url = Url::parse(url).expect("Error parsing url");
    let id = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute resource from url");
    let domain = url.domain().expect("Url has no domain");
    let path = format!("{root_path}/{domain}-{id}.link");
    let mut link = arklib::link::Link::new(url, metadata.title, metadata.desc);
    let write = link.write_to_path(root_path, &path, true);
    tauri::async_runtime::block_on(async { write.await.expect("Writing link to path") });
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_link() {
        let mut cli = Cli::default();
        cli.path = format!(
            "{}/.ark",
            home_dir().expect("Can't find home dir").display()
        );
        cli.link = Some(Link::Add(AddLink {
            url: "http://example.com".into(),
            title: "test".into(),
            description: None,
        }));
        cli.add_new_link();
    }
}
