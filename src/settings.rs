use anyhow::Result;
use config::{Config, Environment, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    // Configure logging level = debug
    #[serde(default = "default_log")]
    pub log: String,
    pub playlists: Vec<Playlist>,
    pub db_url: String,
    pub player_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Playlist {
    pub url: String,
    pub name: String,
}

pub fn default_log() -> String {
    "INFO".to_string()
}

impl Settings {
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Self, config::ConfigError> {
        let mut builder = Config::builder();

        match path {
            Some(file) => {
                builder = builder
                    .add_source(File::with_name(&file.as_ref().to_string_lossy()).required(false));
            }
            None => {
                if let Some(mut path) = home_dir() {
                    path.push(".config");
                    path.push("iptvrs");
                    path.push("settings.toml");
                    builder = builder.add_source(File::from(path).required(false));
                }
            }
        }

        builder
            .add_source(Environment::with_prefix("iptvrs").separator("_"))
            .build()
            .and_then(|config| config.try_deserialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    #[test]
    fn test_settings_from_file() {
        // Create a temporary directory for the test
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("settings.toml");

        // Create a sample settings.toml file
        let sample_settings = r#"
            db_url = "sqlite:///path/to/sqlite.db"
            player_path = "/path/to/player"

            [[playlists]]
            url = "http://example.com/playlist1.m3u"
            name = "Playlist 1"

            [[playlists]]
            url = "http://example.com/playlist2.m3u"
            name = "Playlist 2"
        "#;

        // Write the sample settings to the file
        let mut file = File::create(&file_path).expect("Failed to create settings file");
        file.write_all(sample_settings.as_bytes())
            .expect("Failed to write to settings file");

        // Load the settings from the file
        let settings = Settings::new(Some(&file_path)).expect("Failed to load settings");
        println!("settings: {:#?}", settings);

        // Verify the loaded settings
        assert_eq!(settings.playlists.len(), 2);
        assert_eq!(
            settings.playlists[0].url,
            "http://example.com/playlist1.m3u"
        );
        assert_eq!(settings.playlists[0].name, "Playlist 1");
        assert_eq!(
            settings.playlists[1].url,
            "http://example.com/playlist2.m3u"
        );
        assert_eq!(settings.playlists[1].name, "Playlist 2");
        assert_eq!(settings.db_url, "sqlite:///path/to/sqlite.db");
        assert_eq!(settings.player_path, PathBuf::from("/path/to/player"));
    }
}
