use clap::Clap;

mod args;
mod edit;
mod index;
mod list;
mod new;

use args::{Args, Subcommand};
use edit::EditCommand;
use list::{ListCommand, ListOpts};
use new::NewCommand;

fn main() {
    let cfg = Args::parse();
    match cfg.command {
        Some(Subcommand::Edit(e)) => EditCommand::new(cfg.base_opts, e).run(),
        Some(Subcommand::New(o)) => NewCommand::new(cfg.base_opts, o).run(),
        Some(Subcommand::List(l)) => ListCommand::new(cfg.base_opts, l).run(),
        None => ListCommand::new(cfg.base_opts, ListOpts::default()).run(),
    }
}
