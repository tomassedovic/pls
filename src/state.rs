use crate::show::Show;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt, fs,
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
    pub fn new(config_path: &Path) -> anyhow::Result<Self> {
        let toml = fs::read_to_string(config_path)?;
        let doc = toml.parse::<Document>()?;

        let shows: HashMap<String, Show> = config_path
            .parent()
            // TODO: we'll probably want to actually process the `load_shows` error:
            .and_then(|show_dir| State::load_shows(show_dir).ok())
            .unwrap_or_default();
        log::debug!("Loaded shows: {:#?}", shows);

        // NOTE: Load the `ordering` if it exists in `pls.toml` and
        // use that as the main order in which the shows are listed.
        let mut ordered_keys = doc
            .get("ordering")
            .and_then(toml_edit::Item::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|i| i.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        // Since `ordering` is optional and may not contain all (or
        // any!) of the shows, add in any other shows that we know
        // about.
        let mut show_keys = shows.keys().collect::<Vec<_>>();
        show_keys.sort();
        for key in show_keys {
            if !ordered_keys.contains(key) {
                ordered_keys.push(key.clone());
            }
        }
        log::debug!("Ordered keys: {:#?}", ordered_keys);

        let first_key = ordered_keys.first().cloned().unwrap_or_default();
        log::debug!("First key: {:?}", first_key);

        let version: Option<Version> = doc
            .get("version")
            .and_then(toml_edit::Item::as_str)
            .and_then(Version::from_str);

        let config_version = version.unwrap_or_else(|| {
            let fallback = Version::fallback();
            log::warn!(
                "Unknown or no config version specified. Falling back to: {}",
                fallback
            );
            fallback
        });
        log::info!("Config version: {}", config_version);

        Ok(State {
            config_version,
            selected_key: first_key,
            ordered_keys,
            config_path: config_path.into(),
            config: doc,
            shows,
            error: None,
            about_window_is_open: false,
        })
    }

    pub fn reload_config(&mut self) -> anyhow::Result<()> {
        let new_config = Self::new(&self.config_path)?;
        *self = new_config;
        Ok(())
    }

    pub fn save_config(&self, key: &str) -> anyhow::Result<()> {
        log::info!("Saving config for show: {key}");
        if let (Some(show), Some(config_dir)) = (self.shows.get(key), self.config_path.parent()) {
            let show_path = config_dir.join(format!("{}.{}", key, "toml"));
            log::debug!("Show path: {}", show_path.display());
            let toml_src = fs::read_to_string(&show_path)?;
            let mut doc = toml_src.parse::<Document>()?;
            doc["next"] = toml_edit::value(show.next.display().to_string());
            fs::write(&show_path, doc.to_string())?;
        }
        Ok(())
    }

    pub fn load_shows(show_dir: &Path) -> anyhow::Result<HashMap<String, Show>> {
        let mut shows = HashMap::new();
        for config_path in show_dir.read_dir()? {
            log::debug!("Loading: {:?}", config_path);
            match config_path {
                Ok(config_path) => {
                    if config_path.file_name() == "pls.toml" {
                        log::debug!(
                            "This is the main config file (pls.toml), not a show. Skipping."
                        );
                        continue;
                    } else {
                        log::info!("Loading show at path: {}", config_path.path().display());
                        // TODO: Err handling
                        if let Some(key) = config_path.path().file_stem().and_then(os_to_string) {
                            log::debug!("Show key: {key}");
                            let res = Self::load_show(&config_path.path(), &key).map(|show| {
                                log::debug!("Loaded show: {:#?}", show);
                                shows.insert(key, show);
                            });
                            log::debug!("Load show result: {:?}", res);
                        }
                    }
                }
                Err(error) => {
                    log::error!("Error: {:?}", error);
                }
            }
        }

        Ok(shows)
    }

    pub fn load_show(path: &Path, key: &str) -> anyhow::Result<Show> {
        let toml = fs::read_to_string(path)?;
        let doc = toml.parse::<Document>()?;
        let name = doc.get("name").and_then(|v| v.as_str());
        let dir_default = doc.get("directory").and_then(|v| v.as_str());
        log::debug!("dir_default: {:?}", &dir_default);
        let hostname = hostname::get()
            .ok()
            .and_then(|cstr| cstr.into_string().ok());
        log::debug!("hostname: {:?}", &hostname);
        let dir_hostname = hostname
            .clone()
            .and_then(|hostname| doc.get(&format!("directory_{}", hostname)))
            .map(|v| v.as_str())
            .unwrap_or(dir_default);
        log::debug!("dir_hostname: {:?}", &dir_hostname);

        let name = name.unwrap_or_else(|| {
            log::warn!(
                "The show doesn't have a `name` set. Using the `key` as fallback: `{}`",
                key
            );
            key
        });
        let next = doc.get("next").and_then(|v| v.as_str());

        if let Some(dir) = dir_hostname.and_then(|dir| PathBuf::from(dir).canonicalize().ok()) {
            // Fallback to the first file if no `next` key specified:
            let next = next.map_or_else(
                || {
                    let first = crate::util::all_files_in_dir(&dir)
                        .first()
                        .map(String::from);
                    log::warn!("No `next` key specified for show `{}`", key);
                    log::info!(
                        "Falling back to the first file in the directory: `{:?}`.",
                        first
                    );
                    first
                },
                |s| Some(String::from(s)),
            );

            if let Some(next) = next {
                let next = next.replace(&['\\', '/'][..], std::path::MAIN_SEPARATOR_STR);
                return Ok(Show {
                    name: name.into(),
                    dir,
                    next: next.into(),
                });
            } else {
                log::error!("Error 1: could not load show `{}`:", key);
                log::error!(
                    "No `next` key and couldn't load the first show in directory `{}`",
                    dir.display()
                );
            }
        } else {
            log::error!("Error 2: could not load show `{}`:", key);
            log::error!(
                "Neither the `directory`, nor `directory_{}` key was specified.",
                hostname.unwrap_or_else(|| "hostname".to_string())
            );
        }

        // TODO: replace some of the log issues above with bail as well?
        anyhow::bail!("Could not load show! TODO: better error message.")
    }
}

pub fn os_to_string<T: AsRef<OsStr>>(os_str: T) -> Option<String> {
    os_str.as_ref().to_os_string().into_string().ok()
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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use Version::*;
        let s = match self {
            V1_0_0 => "1.0.0",
        };
        write!(f, "{}", s)
    }
}
