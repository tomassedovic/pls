use std::{fs::File, io::Read};

use toml_edit::Document;

#[derive(Default)]
pub struct State {
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Result<Self, anyhow::Error> {
        let toml = {
            let mut input = String::new();
            File::open("test/pls.toml")?.read_to_string(&mut input)?;
            input
        };
        let doc = toml.parse::<Document>()?;
        dbg!(doc);

        Ok(Self::default())
    }
}
