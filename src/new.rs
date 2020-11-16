use crate::index::Index;
use chrono::*;
use clap::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::args::BaseOpts;

#[derive(Clap)]
pub struct NewOpts {
    name: Option<String>,
}

pub struct NewCommand {
    base_opts: BaseOpts,
    opts: NewOpts,
}

impl NewCommand {
    pub fn new(base_opts: BaseOpts, opts: NewOpts) -> Self {
        NewCommand { base_opts, opts }
    }

    pub fn run(&self) {
        let filepath = self.path();
        let mut handle = Command::new("nvim")
            .arg(&filepath)
            .spawn()
            .expect("failed to start nvim");
        handle.wait().expect("nvim exited with non-zero status");

        let index_path = self.base_opts.index_path();
        let mut index = Index::from_file(&index_path).unwrap_or_default();
        if Path::exists(&filepath) {
            index.add(&filepath);
        }
        index.save(&index_path).expect("failed to save");
    }

    fn path(&self) -> PathBuf {
        let filename = self.opts.name.clone().unwrap_or_else(default_file_name);
        let mut path = self.base_opts.dir.clone();
        path.push(&filename);
        path
    }
}

fn default_file_name() -> String {
    return Local::now().format("%Y%m%d%H%M%S.md").to_string();
}
