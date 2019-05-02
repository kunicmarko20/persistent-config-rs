mod formatter;
mod persistent_config;

pub use crate::persistent_config::PersistentConfig;

#[cfg(test)]
mod tests {
    use config::Config;
    use crate::PersistentConfig;
    use std::fs::File;
    use std::io::prelude::*;
    use crate::formatter::TomlFormatter;

    #[test]
    fn it_works() {
        let mut config = Config::new();
        config.set("test", "Testing").unwrap();
        config.set("test1", "Testing").unwrap();

        assert!(config.save(
            File::create("/tmp/test").unwrap(),
            TomlFormatter::new()
        ).is_ok());

        let mut file = File::open("/tmp/test").expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        assert!(contents.contains("test=\"Testing\"\n"));
        assert!(contents.contains("test1=\"Testing\"\n"));
    }
}
