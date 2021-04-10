#![feature(format_args_capture)]
#![feature(try_blocks)]

use crate::graph::Graph;
use crate::graph::Id;
use crate::view::simple_ui;
use note::Note;

mod filesystem;
pub mod graph;
pub mod note;
pub mod view;

/*
- Each graph (node) is a directory that contains index.md - embeds_dir
- I need navigation: 1) manually 2) tag based
- I need some way to display graph and stuff
   - Display embeds of different kind
- I need data struct to load notes into which is probably most definitely not a graph bc graph ugly
- Maybe pre-calc tag groups?
*/

fn main() {
    let mut g = Graph::<Note>::new();

    g.add_node(Note::from("")).unwrap();
    g.add_node(Note::from("")).unwrap();
    g.remove_node(Id::new(0)).unwrap();
    g.add_node(Note::from("")).unwrap();

    simple_ui()
}
