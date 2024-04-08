use crate::{db::DB, entry::IptvEntry, settings::Settings};
use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use tracing::info;

/// Hydrate the database with the contents of playlist
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db = DB::from_settings(settings).await?;
        let client = Client::new();
        for (playlist_index, playlist) in settings.playlists.iter().enumerate() {
            let playlist_name = &playlist.name;
            info!(playlist_name = ?playlist_name, "populating playlist");
            let resp = client.get(&playlist.url).send().await?;
            let body = resp.text().await?;
            let iptv_entries = parse_m3u(playlist_name, playlist_index as u32, &body)?;
            db.insert_or_update_entries(iptv_entries).await?;
        }

        Ok(())
    }
}

fn parse_m3u(
    playlist_name: &str,
    playlist_index: u32,
    m3u_content: &str,
) -> Result<Vec<IptvEntry>> {
    let mut entries = Vec::new();
    let mut current_name = String::new();

    for (index, line) in m3u_content.lines().enumerate() {
        if line.starts_with("#EXTINF:") {
            let parts: Vec<&str> = line.split(',').collect();
            current_name = parts.last().unwrap_or(&"").to_string();
        } else if !line.starts_with('#') && !line.is_empty() {
            entries.push(IptvEntry {
                channel_index: playlist_index + index as u32,
                channel_name: current_name.clone(),
                channel_url: line.to_string(),
                playlist_name: playlist_name.to_string(),
            });
        }
    }

    Ok(entries)
}
