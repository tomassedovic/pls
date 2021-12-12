use std::error::Error;

#[derive(Default)]
pub struct State {
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        use toml_edit::{Document, Value};
        let toml = {
            use std::io::Read;
            let mut input = String::new();
            std::fs::File::open("test/pls.toml")?.read_to_string(&mut input)?;
            input
        };
        let mut doc = toml.parse::<Document>()?;
        dbg!(doc);

        Ok(Self::default())
    }
}
