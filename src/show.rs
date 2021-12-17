use std::path::Path;

#[derive(Debug)]
pub struct Show {
    pub name: String,
}

impl Show {
    pub fn current_episode(&self) -> &Path {
        Path::new("/home/shadower/Videos/Bleach/Bleach - 01.mkv")
    }

    pub fn next(&mut self) {
        todo!()
    }
}
