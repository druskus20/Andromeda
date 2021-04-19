#![feature(format_args_capture)]
#![feature(try_blocks)]

use crate::opts::{Action::*, Opt};
use crate::view::simple_ui;
use anyhow::bail;
use note::Note;
use structopt::StructOpt;

mod config;
pub mod note;
mod note_collection;
mod opts;
pub mod view;

/*
- Each graph (node) is a directory that contains index.md - embeds_dir
- I need navigation: 1) manually 2) tag based
- I need some way to display graph and stuffr
   - Display embeds of different kind
- I need data struct to load notes into which is probably most definitely not a graph bc graph ugly
- Maybe pre-calc tag groups?
*/

fn main() {
    let opts = Opt::from_args();

    match opts.action {
        CreateNote { name } => println!("CreateNode {}", name),
        _ => {
            eprintln!("Action not supported");
            std::process::exit(1);
        }
    };
}
