use crate::show::Show;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml_edit::Document;

#[derive(Debug)]
pub struct State {
    pub selected_key: String,
    pub ordered_keys: Vec<String>,
    pub config: Document,
    pub config_path: PathBuf,
    pub shows: HashMap<String, Show>,
    pub error: Option<String>,
}

impl State {
    pub fn new(config_path: &Path) -> Result<Self, anyhow::Error> {
        let toml = std::fs::read_to_string(config_path)?;
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
            let name = show.get("name").and_then(|v| v.as_str());
            let dir_default = show.get("directory").and_then(|v| v.as_str());
            let hostname = hostname::get()
                .ok()
                .and_then(|cstr| cstr.into_string().ok());
            let dir_hostname = hostname
                .clone()
                .and_then(|hostname| show.get(format!("directory_{}", hostname)))
                .map(|v| v.as_str())
                .unwrap_or(dir_default);

            let name = name.unwrap_or_else(|| {
                eprintln!(
                    "Warning: the show doesn't have a `name` set. Using the `key` as fallback: `{}`",
                    key
                );
                key
            });
            let next = show.get("next").and_then(|v| v.as_str());

            if let Some(dir) = dir_hostname {
                let dir = PathBuf::from(dir);
                // Fallback to the first file if no `next` key specified:
                let next = next.map_or_else(
                    || {
                        let first = crate::util::all_files_in_dir(&dir)
                            .first()
                            .map(String::from);
                        eprintln!("Warning: no `next` key specified for show `{}`", key);
                        println!(
                            "Falling back to the first file in the directory: `{:?}`.",
                            first
                        );
                        first
                    },
                    |s| Some(String::from(s)),
                );

                if let Some(next) = next {
                    let next =
                        next.replace(&['\\', '/'][..], &std::path::MAIN_SEPARATOR.to_string());
                    let show = Show {
                        name: name.into(),
                        dir,
                        next: next.into(),
                    };
                    shows.insert(key.to_string(), show);
                } else {
                    eprintln!("Error: could not load show `{}`:", key);
                    eprintln!(
                        "No `next` key and couldn't load the first show in directory `{}`",
                        dir.display()
                    );
                }
            } else {
                eprintln!("Error: could not load show `{}`:", key);
                if dir_hostname.is_none() {
                    eprintln!(
                        "Neither the `directory`, nor `directory_{}` key was specified.",
                        hostname.unwrap_or_else(|| "hostname".to_string())
                    );
                }
            }
        }

        Ok(State {
            selected_key: first_key.to_string(),
            ordered_keys,
            config_path: config_path.into(),
            config: doc,
            shows,
            error: None,
        })
    }

    pub fn reload_config(&mut self) -> Result<(), anyhow::Error> {
        let new_config = Self::new(&self.config_path)?;
        *self = new_config;
        Ok(())
    }

    pub fn save_config(&self) {
        let _ = std::fs::write(&self.config_path, self.config.to_string());
    }
}
