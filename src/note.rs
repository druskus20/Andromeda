use crate::graph::{HasId, Id};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Note {
    pub id: Id,
    pub title: String,
    pub path: PathBuf,
    pub tags: Vec<String>,
}

// Impl From<P> for Note where P implements AsRef<Path>
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
}

impl HasId for Note {
    fn get_id(&self) -> Id {
        self.id
    }
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
