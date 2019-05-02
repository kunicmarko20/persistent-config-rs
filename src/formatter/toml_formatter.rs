use config::Value;
use crate::formatter::Formatter;

pub struct TomlFormatter;

impl TomlFormatter {
    pub fn new() -> Self {
        TomlFormatter
    }
}

impl Formatter for TomlFormatter {
    fn format(&self, key: String, value: Value) -> String {
        format!("{}=\"{}\"\n", key, value.into_str().expect("Unable to convert a value to a string."))
    }
}
