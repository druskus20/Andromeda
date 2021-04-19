use std::path::{Path, PathBuf};

use self::graph::Graph;
use crate::note::Note;
use anyhow::*;

// This should be an abstraction layer over graph
mod graph;

#[derive(Debug, Clone)]
pub struct NoteCollection {
    graph: Graph<Note>,
    graph_path: PathBuf,
    notes_path: PathBuf,
}

impl NoteCollection {
    pub fn new(graph_path: impl AsRef<Path>, notes_path: impl AsRef<Path>) -> Self {
        NoteCollection {
            graph: Graph::<Note>::new(),
            graph_path: graph_path.as_ref().to_path_buf(),
            notes_path: notes_path.as_ref().to_path_buf(),
        }
    }

    pub fn add_note(&mut self, name: &str) -> Result<()> {
        let n: Note = Note::new();
        self.graph.add_node(n)?;

        let note_dir = self.notes_path.join(name);
        // !! std::fs::File::create(note_dir)?;

        // Open file in $EDITOR
        dbg!("fuck windows");

        Ok(())
    }

    pub fn remove_note(&mut self, name: &str) -> Result<()> {
        let note_dir = self.notes_path.join(name);
        let metadata_path = std::fs::read_dir(note_dir.clone())?
            .filter_map(|x| x.ok())
            .find(|x| x.file_name() == ".metadata")
            .with_context(|| {
                format!(".metadata could not be found inside {}", note_dir.display())
            })?;

        let metdata = std::fs::read_to_string(metadata_path.path())?;
    
        // TODO: apply regex and load it into a dict or map
        
        // TODO:
        //self.graph.remove_node(id)?;
        // remove directory

        Ok(())
    }
}
