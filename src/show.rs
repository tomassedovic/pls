use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Show {
    pub name: String,
    pub dir: PathBuf,
    pub next: PathBuf,
}

impl Show {
    pub fn current_episode(&self) -> PathBuf {
        self.dir.join(&self.next)
    }

    pub fn next(&mut self) {
        todo!()
    }
}
