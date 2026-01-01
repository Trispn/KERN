#[derive(Debug, Clone)]
pub struct Config {
    pub max_errors: usize,
    pub enable_unicode: bool,
    pub tab_width: usize,
    pub max_source_lines: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_errors: 100,
            enable_unicode: true,
            tab_width: 4,
            max_source_lines: 10000,
        }
    }
}