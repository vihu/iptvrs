use crate::settings::Settings;
use anyhow::Result;
use clap::Parser;

/// List all channels in the database
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, settings: &Settings) -> Result<()> {
        let db_path = &settings.db_path;

        let db = sled::open(db_path)?;

        let lookup_tree = db.open_tree("lookup")?;
        let channel_tree = db.open_tree("channel")?;

        let mut results = vec![];
        for entry in lookup_tree.iter() {
            let (key, value) = entry?;
            let channel_name = String::from_utf8(key.to_vec())?;
            let channel_index = String::from_utf8(value.to_vec())?;
            if let Ok(Some(_channel_url)) = channel_tree.get(&channel_index) {
                results.push((channel_index.parse::<u32>()?, channel_name));
            }
        }

        results.sort_by(|a, b| a.0.cmp(&b.0));

        for (channel_index, channel_name) in results {
            println!("{} \t {}", channel_index, channel_name);
        }

        Ok(())
    }
}
