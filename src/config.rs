#[derive(Debug)]
pub struct Config {
    process: bool,
    empty_symbol: String,
    uncertain_symbol: String,
    filled_symbol: String,
}

impl Config {
    pub fn new(
        process: bool,
        empty_symbol: String,
        uncertain_symbol: String,
        filled_symbol: String,
    ) -> Self {
        Self {
            process,
            empty_symbol,
            uncertain_symbol,
            filled_symbol,
        }
    }

    pub fn process(&self) -> bool {
        self.process
    }

    pub fn empty_symbol(&self) -> &str {
        &self.empty_symbol
    }

    pub fn uncertain_symbol(&self) -> &str {
        &self.uncertain_symbol
    }

    pub fn filled_symbol(&self) -> &str {
        &self.filled_symbol
    }
}
