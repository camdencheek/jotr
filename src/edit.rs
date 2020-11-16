use crate::args::BaseOpts;
use crate::index::Index;
use clap::*;
use std::process::Command;
use std::usize;

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
        let mut index = Index::from_file(&index_path).unwrap_or_default();

        let entry = if let Ok(id) = usize::from_str_radix(&self.opts.name, 10) {
            index.get(id)
        } else {
            index.get_by_name(&self.opts.name)
        };

        let entry = match entry {
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
