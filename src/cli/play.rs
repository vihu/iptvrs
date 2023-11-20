use crate::settings::Settings;
use anyhow::Result;
use clap::Parser;
use std::process::{Command, Stdio};

/// Play a channel using the configured player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {
    /// The channel index to play
    channel_index: u32,
}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db_path = &settings.db_path;

        let db = sled::open(db_path)?;
        let channel_index = self.channel_index.to_string();

        let channel_tree = db.open_tree("channel")?;

        if let Ok(Some(channel_url)) = channel_tree.get(channel_index) {
            let channel_url_str = String::from_utf8(channel_url.to_vec())?;
            Command::new(&settings.player_path)
                .arg(&channel_url_str)
                .arg("--quiet")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
        }

        Ok(())
    }
}
