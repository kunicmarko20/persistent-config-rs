use std::io::Write;
use config::{Config, Source};
use crate::formatter::Formatter;

pub trait PersistentConfig {
    fn save<W: Write, F: Formatter>(&self, writer: W, formatter: F) -> Result<(), String>;
}

impl PersistentConfig for Config {
    fn save<W: Write, F: Formatter>(&self, mut writer: W, formatter: F) -> Result<(), String> {
        for (key, value) in self.collect().expect("Unable to collect config.") {
            writer.write(
                formatter.format(
                    key,
                    value
                ).as_bytes()
            ).expect("Unable to write config.");
        }

        Ok(())
    }
}
