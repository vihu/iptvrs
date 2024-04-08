use crate::{db::DB, settings::Settings};
use anyhow::Result;
use clap::Parser;

/// List all channels in the database
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db = DB::from_settings(settings).await?;
        let entries = db.list_entries().await?;

        let json = serde_json::to_string_pretty(&entries)?;
        println!("{}", json);

        Ok(())
    }
}
