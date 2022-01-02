use crate::show::Show;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml_edit::Document;

#[derive(Debug)]
pub struct State {
    pub config_version: Version,
    pub selected_key: String,
    pub ordered_keys: Vec<String>,
    pub config: Document,
    pub config_path: PathBuf,
    pub shows: HashMap<String, Show>,
    pub error: Option<String>,
    pub about_window_is_open: bool,
}

impl State {
    pub fn new(config_path: &Path) -> Result<Self, anyhow::Error> {
        let toml = std::fs::read_to_string(config_path)?;
        let doc = toml.parse::<Document>()?;
        let first_key = doc
            .iter()
            .filter(|(_key, value)| value.is_table())
            .map(|(key, _series)| key)
            .next()
            .unwrap_or_default();
        println!("First key: {:?}", first_key);
        let mut version = None;
        let mut ordered_keys = vec![];
        let mut shows = HashMap::new();
        for (key, value) in doc.iter() {
            if value.is_table() {
                ordered_keys.push(key.to_string());
                let name = value.get("name").and_then(|v| v.as_str());
                let dir_default = value.get("directory").and_then(|v| v.as_str());
                let hostname = hostname::get()
                    .ok()
                    .and_then(|cstr| cstr.into_string().ok());
                let dir_hostname = hostname
                    .clone()
                    .and_then(|hostname| value.get(format!("directory_{}", hostname)))
                    .map(|v| v.as_str())
                    .unwrap_or(dir_default);

                let name = name.unwrap_or_else(|| {
                eprintln!(
                    "Warning: the show doesn't have a `name` set. Using the `key` as fallback: `{}`",
                    key
                );
                key
            });
                let next = value.get("next").and_then(|v| v.as_str());

                if let Some(dir) =
                    dir_hostname.and_then(|dir| PathBuf::from(dir).canonicalize().ok())
                {
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
            } else if key == "version" {
                // TODO: Have `version` be a `Result`, record if it's an unexpected type, unknown value or unspecified
                if let Some(version_str) = value.as_str() {
                    version = Version::from_str(version_str);
                }
            }
        }

        let config_version = version.unwrap_or_else(|| {
            let fallback = Version::fallback();
            eprintln!(
                "Warning: unknown or no config version specified. Falling back to: {}",
                fallback
            );
            fallback
        });
        println!("Config version: {}", config_version);

        Ok(State {
            config_version,
            selected_key: first_key.to_string(),
            ordered_keys,
            config_path: config_path.into(),
            config: doc,
            shows,
            error: None,
            about_window_is_open: false,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Version {
    V1_0_0,
}

impl Version {
    pub fn from_str(version_str: &str) -> Option<Self> {
        match version_str {
            "1.0.0" => Some(Version::V1_0_0),
            _ => None,
        }
    }

    pub fn fallback() -> Self {
        Version::V1_0_0
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Version::*;
        let s = match self {
            V1_0_0 => "1.0.0",
        };
        write!(f, "{}", s)
    }
}
