use crate::{db::DB, settings::Settings};
use anyhow::Result;
use clap::Parser;

/// Search the database for a channel
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {
    /// The name of the channel to search for
    channel_name: String,
}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db = DB::from_settings(settings).await?;
        let matches = db.search_entries(&self.channel_name).await?;
        let json = serde_json::to_string_pretty(&matches)?;
        println!("{}", json);

        Ok(())
    }
}
