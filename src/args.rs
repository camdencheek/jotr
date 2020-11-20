use crate::delete;
use crate::edit;
use crate::list;
use crate::new;
use std::path::PathBuf;

use clap::*;

#[derive(Clap)]
#[clap(author = "Camden Cheek <camden@ccheek.com>")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Subcommand>,

    #[clap(flatten)]
    pub base_opts: BaseOpts,
}

#[derive(Clap)]
pub enum Subcommand {
    #[clap(alias = "e")]
    Edit(edit::EditOpts),

    #[clap(alias = "n")]
    New(new::NewOpts),

    #[clap(alias = "l")]
    List(list::ListOpts),

    #[clap(alias = "d")]
    Delete(delete::DeleteOpts),
}

#[derive(Clap)]
pub struct BaseOpts {
    #[clap(long, short, default_value = default_dir())]
    pub dir: PathBuf,

    #[clap(long, default_value = "vi", env = "EDITOR")]
    pub editor: String,
}

impl BaseOpts {
    pub fn index_path(&self) -> PathBuf {
        let mut new = self.dir.clone();
        new.push(".index");
        new
    }
}

fn default_dir() -> &'static str {
    // TODO
    "/Users/ccheek/notes"
}
