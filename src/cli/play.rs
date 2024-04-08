use crate::{db::DB, settings::Settings};
use anyhow::Result;
use clap::Parser;
use std::process::{Command, Stdio};
use tracing::{error, info, warn};

/// Play a channel using the configured player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {
    /// The channel index to play
    channel_index: usize,
}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db = DB::from_settings(settings).await?;
        let channel_index = self.channel_index;

        match db.get_entry_by_channel_index(channel_index).await {
            Ok(Some(entry)) => {
                info!(channel_index = ?channel_index, channel_name = ?entry.channel_name, "playing selected channel");
                let channel_url = entry.channel_url;
                Command::new(&settings.player_path)
                    .arg(&channel_url)
                    .arg("--quiet")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?;
            }
            Ok(None) => warn!(channel_index = ?channel_index, "no entry found"),
            Err(e) => error!(error = ?e, "error"),
        }

        Ok(())
    }
}
