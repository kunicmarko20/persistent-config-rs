use config::Value;

pub trait Formatter {
    fn format(&self, key: String, value: Value) -> String;
}
