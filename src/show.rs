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

    pub fn advance_to_next_episode(&mut self) {
        let eps = self.episodes();
        let mut iter = eps.iter();
        let current_episode = self.current_episode();
        // Find the current episode
        iter.find(|episode| episode == &&current_episode);
        // Get the next one:
        let next_episode = iter.next();
        if let Some(next) = next_episode {
            let full = PathBuf::from(next);
            let stripped = full.strip_prefix(&self.dir);
            self.next = stripped.map(Path::to_path_buf).unwrap_or(full);
        }
    }

    pub fn previous_episode(&self) -> Option<PathBuf> {
        let eps = self.episodes();
        let current_episode = self.current_episode();
        let current_episode_index = eps.iter().position(|episode| episode == &current_episode);
        if let Some(index) = current_episode_index
            && index > 0
        {
            let previous_episode = &eps[index - 1];
            return Some(self.dir.join(previous_episode));
        }
        None
    }

    pub fn episodes(&self) -> Vec<PathBuf> {
        crate::util::all_paths_in_dir(&self.dir)
    }
}
