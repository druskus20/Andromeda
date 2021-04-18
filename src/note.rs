/*
   Idea is:
       - There's a pile of notes unorganized in one directory
       - There are separate directories for each tag, that contain symlinks to notes in the pile directory
*/

use crate::graph::{HasId, Id};
use std::path::{Path, PathBuf};

// TODO: Consider removing id and using node instead
#[derive(Debug, Clone)]
pub struct Note {
    pub title: String,
    pub path: PathBuf,
    pub tags: Vec<String>,
}

// Impl From<P> for Note where P implements AsRef<Path>
/*
impl<P: AsRef<Path>> From<P> for Note {
    fn from(path: P) -> Self {
        let path = path.as_ref();

        Note {
            id: Id::new(0),
            title: "".to_string(),
            path: PathBuf::from(path),
            tags: vec![],
        }
    }
}*/

impl Note {
    pub fn new() -> Self {
        Note {
            title: "".to_string(),
            path: PathBuf::from(""),
            tags: vec![],
        }
    }
}



