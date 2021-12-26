use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use humanesort::HumaneOrder;

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

    pub fn advance_to_next_episode(&mut self) {
        let eps = self.episodes();
        let mut iter = eps.iter();
        //let current_episode = self.current_episode().display().to_string();
        let current_episode = self.current_episode();
        // Find the current episode
        iter.find(|episode| episode == &&current_episode);
        // Get the next one:
        let next_episode = iter.next();
        dbg!(next_episode);
        if let Some(next) = next_episode {
            let full = PathBuf::from(next);
            let stripped = full.strip_prefix(&self.dir);
            self.next = stripped.map(Path::to_path_buf).unwrap_or(full);
        }
    }

    pub fn episodes(&self) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = vec![];
        let _ = visit_dirs(&self.dir, &mut |entry: &DirEntry| {
            let path: PathBuf = entry.path();
            assert!(path.starts_with(&self.dir));
            result.push(path);
        });

        result.sort_by(|a, b| {
            HumaneOrder::humane_cmp(&a.display().to_string(), &b.display().to_string())
        });

        result
    }
}

/// Adapted from: https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
