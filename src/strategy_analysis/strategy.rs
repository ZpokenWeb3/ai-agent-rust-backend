use std::fs;
use std::path::Path;

/// Represents a strategy with trading rules.
pub struct Strategy {
    pub rules: String,
}

impl Strategy {
    /// Loads a strategy configuration file.
    pub fn from_file(filename: &str) -> Self {
        let path = Path::new("src/strategy_analysis/prompts").join(filename);
        let strategy_config = fs::read_to_string(&path)
            .expect("Failed to load strategy configuration file.");
        Self { rules: strategy_config }
    }
}
