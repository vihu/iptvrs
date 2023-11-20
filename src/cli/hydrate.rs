use crate::{entry::IptvEntry, settings::Settings};
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

/// Hydrate the database with the contents of playlist
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db_path = &settings.db_path;

        let client = Client::new();
        let resp = client.get(&settings.url).send().await?;

        let body = resp.text().await?;
        let iptv_entries = parse_m3u(&body)?;

        let db = sled::open(db_path)?;
        let lookup_tree = db.open_tree("lookup")?;
        let channel_tree = db.open_tree("channel")?;

        for (channel_index, entry) in iptv_entries.iter().enumerate() {
            let channel_index = channel_index.to_string();
            let channel_name = entry.name.to_lowercase();
            let channel_url = &entry.url;

            lookup_tree.insert(channel_name, channel_index.as_bytes())?;
            channel_tree.insert(channel_index, channel_url.as_bytes())?;
        }

        Ok(())
    }
}

fn parse_m3u(m3u_content: &str) -> Result<Vec<IptvEntry>> {
    let mut entries = Vec::new();
    let mut current_name = String::new();

    for line in m3u_content.lines() {
        if line.starts_with("#EXTINF:") {
            let parts: Vec<&str> = line.split(',').collect();
            current_name = parts.last().unwrap_or(&"").to_string();
        } else if !line.starts_with('#') && !line.is_empty() {
            entries.push(IptvEntry {
                name: current_name.clone(),
                url: line.to_string(),
            });
        }
    }

    Ok(entries)
}
