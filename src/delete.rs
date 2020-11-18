use crate::index::Index;
use clap::*;
use std::fs;

use crate::args::BaseOpts;

#[derive(Clap)]
pub struct DeleteOpts {
    name: String,
}

pub struct DeleteCommand {
    base_opts: BaseOpts,
    opts: DeleteOpts,
}

impl DeleteCommand {
    pub fn new(base_opts: BaseOpts, opts: DeleteOpts) -> Self {
        DeleteCommand { base_opts, opts }
    }

    pub fn run(&self) {
        let index_path = self.base_opts.index_path();
        let mut index = Index::from_file(&index_path).expect("failed to read from index");

        let entry = match index.get(&self.opts.name) {
            Some(e) => e,
            None => {
                println!("Entry '{}' not found", &self.opts.name);
                return;
            }
        };

        fs::remove_file(&entry.path).expect("failed to delete");

        index.delete(&entry);
        index.save(&index_path).expect("failed to save");
    }
}
