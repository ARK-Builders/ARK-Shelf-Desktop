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
                    create_link(l.title.clone(), l.description.clone(), &l.url, &self.path)
                        .expect("Creating Link");
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
    title: String,
    desc: Option<String>,
    url: &str,
    root_path: &str,
) -> Result<(), String> {
    let url = match Url::parse(url) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    let resource = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute resource from url");
    let domain = url.domain().expect("Url has no domain");
    let path = format!("{}/{domain}-{}.link", root_path, resource.crc32);
    let mut link = arklib::link::Link::new(url, title, desc);
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