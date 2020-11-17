use clap::*;
use colored::*;
use prettytable::{cell, format, row, Table};

use crate::args::BaseOpts;
use crate::index::Index;

#[derive(Clap)]
pub struct ListOpts {
    #[clap(short, long)]
    limit: Option<usize>,
}

pub struct ListCommand {
    base_opts: BaseOpts,
    opts: ListOpts,
}

impl ListCommand {
    pub fn new(base_opts: BaseOpts, opts: ListOpts) -> Self {
        ListCommand { base_opts, opts }
    }

    pub fn run(&self) {
        let index_path = self.base_opts.index_path();
        let index = Index::from_file(&index_path).unwrap_or_default();
        let notes = index.sorted_most_recent(self.opts.limit);

        let mut table = Table::new();
        let fmt = format::FormatBuilder::new().padding(0, 1).build();
        table.set_format(fmt);
        for entry in notes {
            table.add_row(row![
                format!("[{}]", entry.id.to_string().blue()),
                entry.summarize(),
            ]);
        }
        table.printstd();
    }
}

impl Default for ListOpts {
    fn default() -> Self {
        ListOpts { limit: None }
    }
}
