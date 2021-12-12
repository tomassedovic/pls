use std::{fs::File, io::Read};

use toml_edit::Document;

pub struct State {
    pub selected_key: String,
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
        let first_key = doc
            .iter()
            .next()
            .map(|(key, _series)| key)
            .unwrap_or_default();

        Ok(State {
            selected_key: first_key.to_string(),
            config: doc,
        })
    }
}
