use toml_edit::Document;

pub struct State {
    pub selected_key: String,
    pub config: Document,
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

        Ok(State {
            selected_key: first_key.to_string(),
            config: doc,
        })
    }
}
