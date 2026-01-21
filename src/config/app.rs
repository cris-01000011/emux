use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TomlConfig {
    app: PathsConfig,
}

#[derive(Deserialize, Debug)]
struct PathsConfig {
    base_dir: String,
}

pub struct AppConfig;

impl AppConfig {
    pub fn base_dir() -> PathBuf {
        let Some(home) = home::home_dir() else {
            return PathBuf::from("./Emux");
        };

        let config_path = home.join(".config/emux/emux.toml");

        let Ok(config_content) = std::fs::read_to_string(&config_path) else {
            return home.join("Emux");
        };

        let Ok(config) = toml::from_str::<TomlConfig>(&config_content) else {
            return home.join("Emux");
        };

        if config.app.base_dir != "default" {
            return PathBuf::from(config.app.base_dir);
        }

        home.join("Emux")
    }

    pub fn lists_dir() -> PathBuf {
        Self::base_dir().join("lists")
    }
}
