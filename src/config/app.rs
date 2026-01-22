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

#[derive(Default)]
pub struct AppConfig {
    pub base_dir: PathBuf,
    pub lists_dir: PathBuf,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            base_dir: Self::get_base_dir(),
            lists_dir: Self::get_lists_dir(),
        }
    }

    pub fn get_base_dir() -> PathBuf {
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

    fn get_lists_dir() -> PathBuf {
        Self::get_base_dir().join("lists")
    }
}
