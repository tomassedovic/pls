use crate::show::Show;

use std::{collections::HashMap, path::PathBuf};

use toml_edit::Document;

#[derive(Debug)]
pub struct State {
    pub selected_key: String,
    pub config: Document,
    pub shows: HashMap<String, Show>,
}

impl State {
    pub fn new() -> Result<Self, anyhow::Error> {
        let toml = std::fs::read_to_string("test/pls.toml")?;
        let doc = toml.parse::<Document>()?;
        let first_key = doc
            .iter()
            .next()
            .map(|(key, _series)| key)
            .unwrap_or_default();

        let mut shows = HashMap::new();
        for (key, show) in doc.iter() {
            let name = show.get("name").map(|v| v.as_str()).flatten();
            let dir = show.get("directory").map(|v| v.as_str()).flatten();
            let next = show.get("next").map(|v| v.as_str()).flatten();
            if let (Some(name), Some(dir), Some(next)) = (name, dir, next) {
                let show = Show {
                    name: name.to_string(),
                    dir: PathBuf::from(dir),
                    next: PathBuf::from(next),
                };
                shows.insert(key.to_string(), show);
            }
        }

        Ok(State {
            selected_key: first_key.to_string(),
            config: doc,
            shows,
        })
    }
}
