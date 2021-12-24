use crate::show::Show;

use std::{collections::HashMap, path::PathBuf};

use toml_edit::Document;

#[derive(Debug)]
pub struct State {
    pub selected_key: String,
    pub ordered_keys: Vec<String>,
    pub config: Document,
    pub shows: HashMap<String, Show>,
    pub error: Option<String>,
}

impl State {
    pub fn new() -> Result<Self, anyhow::Error> {
        // TODO: get the config location passed from main
        let toml = std::fs::read_to_string("test/pls.toml")?;
        let doc = toml.parse::<Document>()?;
        let first_key = doc
            .iter()
            .next()
            .map(|(key, _series)| key)
            .unwrap_or_default();

        let mut ordered_keys = vec![];
        let mut shows = HashMap::new();
        for (key, show) in doc.iter() {
            ordered_keys.push(key.to_string());
            let name = show.get("name").map(|v| v.as_str()).flatten();
            let dir_default = show.get("directory").map(|v| v.as_str()).flatten();
            let hostname = hostname::get()
                .ok()
                .map(|cstr| cstr.into_string().ok())
                .flatten();
            dbg!(&hostname);
            let dir_hostname = hostname
                .map(|hostname| {
                    show.get(format!("directory_{}", hostname))
                        .map(|v| v.as_str())
                })
                .flatten()
                .unwrap_or(dir_default);
            dbg!(&key);
            dbg!(&dir_default);
            dbg!(&dir_hostname);

            let next = show.get("next").map(|v| v.as_str()).flatten();
            if let (Some(name), Some(dir), Some(next)) = (name, dir_hostname, next) {
                let next = next.replace(&['\\', '/'][..], &std::path::MAIN_SEPARATOR.to_string());
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
            ordered_keys,
            config: doc,
            shows,
            error: None,
        })
    }
}
