use arklib::link::Metadata;
use clap::{Parser, Subcommand};
use home::home_dir;
use url::Url;

#[derive(Parser, Default, Debug)]
#[clap(
    name = "ARK Shelf Desktop",
    about = "Desktop Version of ARK Shelf, put you bookmarks when surfing."
)]
pub struct Cli {
    #[clap(
        short, long, help = "Path to store .link file", 
        default_value_t = format!("{}/.ark-shelf",home_dir().expect("Can't find home dir").display())
    )]
    pub path: String,
    #[clap(subcommand)]
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
                    create_link(&l.url, &self.path, metadata).expect("Creating Link");
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Subcommand, Debug)]
pub enum Link {
    /// Adds a new link
    Add(AddLink),
}

#[derive(Parser, Debug)]
pub struct AddLink {
    #[clap(short, long)]
    pub url: String,

    #[clap(short, long)]
    pub title: String,

    #[clap(short, long)]
    pub description: Option<String>,
}

/// Creates a `.link`
pub fn create_link(
    url: &str,
    root_path: &str,
    metadata: arklib::link::Metadata,
) -> Result<(), String> {
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
            "{}/.ark-shelf",
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
