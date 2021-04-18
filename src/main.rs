#![feature(format_args_capture)]
#![feature(try_blocks)]

use crate::graph::Graph;
use crate::graph::Id;
use crate::view::simple_ui;
use anyhow::bail;
use note::Note;
use crate::opts::{
    Opt,
    Action::*,
};
use structopt::StructOpt;


pub mod graph;
pub mod note;
pub mod view;
mod config;
mod opts;

/*
- Each graph (node) is a directory that contains index.md - embeds_dir
- I need navigation: 1) manually 2) tag based
- I need some way to display graph and stuffr
   - Display embeds of different kind
- I need data struct to load notes into which is probably most definitely not a graph bc graph ugly
- Maybe pre-calc tag groups?
*/

fn main() {
    let mut g = Graph::<Note>::new();
    let opts = Opt::from_args();

    match opts.action {
        CreateNote { name } => println!("CreateNode {}", name),
        _ => {
            eprintln!("Action not supported");
            std::process::exit(1);
        }
    };
    
    Note::new();
   
    g.add_node(Note::new()).unwrap();
    g.add_node(Note::new()).unwrap();
    g.remove_node(Id::new(0)).unwrap();
    g.add_node(Note::new()).unwrap();
}
