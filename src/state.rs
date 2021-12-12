use std::{fs::File, io::Read};

use toml_edit::Document;

pub struct State {
    pub selected_index: usize,
    pub config: Document,
}

impl State {
    pub fn new() -> Result<Self, anyhow::Error> {
        let toml = {
            let mut input = String::new();
            File::open("test/pls.toml")?.read_to_string(&mut input)?;
            input
        };
        let doc = toml.parse::<Document>()?;

        Ok(State {
            selected_index: 0,
            config: doc,
        })
    }
}
