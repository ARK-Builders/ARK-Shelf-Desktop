use clap::{Parser, Subcommand};
use home::home_dir;
use url::Url;
use tokio::runtime::Runtime;


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
    pub link: Option<Link>
}

impl Cli {
    pub fn add_new_link(&self) -> bool {
        if let Some(link) = &self.link {
            match link {
                Link::Add(l) => {
                    let title = l.title.clone();
                    let desc = l.description.clone();
                    let url = l.url.clone();
                    create_link(title, desc, url, self.path.clone()).expect("Creating Link");
                    return true
                }
            }
        } 
        return false
    }
}

#[derive(Subcommand, Debug)]
pub enum Link {
    /// Adds a new link
    Add(AddLink)
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
/// 
/// Modified version of `command::create_link` which can't be reused as 
/// there's no way to construct `tauri::State`
fn create_link(
    title: String,
    desc: Option<String>,
    url: String,
    cli_path: String,
) -> Result<(), String> {
    let url = match Url::parse(url.as_str()) {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };
    let resource = arklib::id::ResourceId::compute_bytes(url.as_ref().as_bytes())
        .expect("Error compute resource from url");
    let domain = url.domain().expect("Url has no domain");
    let path = format!("{}/{domain}-{}.link", cli_path.clone(), resource.crc32);
    let mut link = arklib::link::Link::new(url, title, desc);
    let rt  = Runtime::new().map_err(|_| "Creating runtime")?;
    let write = link.write_to_path(cli_path, path, true);
    rt.block_on(async { write.await.expect("Writing link to path"); });
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_link() {
        let mut cli = Cli::default();
        cli.path = format!("{}/.ark-shelf",home_dir().expect("Can't find home dir").display());
        cli.link = Some(Link::Add( AddLink {
            url: "http://example.com".into(),
            title: "test".into(),
            description: None
        }));
        cli.add_new_link();
    }
}