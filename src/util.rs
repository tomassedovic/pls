use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use humanesort::HumaneOrder;

pub fn all_paths_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut result = vec![];
    let _ = visit_dirs(dir, &mut |entry| {
        let path: PathBuf = entry.path();
        assert!(path.starts_with(dir));
        result.push(path);
    });

    result.sort_by(|a, b| {
        HumaneOrder::humane_cmp(&a.display().to_string(), &b.display().to_string())
    });

    result
}

pub fn all_files_in_dir(dir: &Path) -> Vec<String> {
    use humanesort::prelude::*;
    let mut result = vec![];
    let _ = visit_dirs(dir, &mut |entry| {
        let path: PathBuf = entry.path();
        assert!(path.starts_with(dir));
        result.push(path.display().to_string());
    });

    result.humane_sort();

    result
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
