use crate::args::BaseOpts;
use crate::index::Index;
use clap::*;
use std::process::Command;

#[derive(Clap)]
pub struct EditOpts {
    name: String,
}

pub struct EditCommand {
    base_opts: BaseOpts,
    opts: EditOpts,
}

impl EditCommand {
    pub fn new(base_opts: BaseOpts, opts: EditOpts) -> Self {
        EditCommand { base_opts, opts }
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

        let mut handle = Command::new("nvim")
            .arg(&entry.path)
            .spawn()
            .expect("failed to start nvim");
        handle.wait().expect("nvim exited with non-zero status");

        index.touch(entry.id);
        index.save(index_path).expect("failed to save index");
    }
}
