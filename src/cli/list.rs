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

        for entry in lookup_tree.iter() {
            let (key, value) = entry?;
            let key_str = String::from_utf8(key.to_vec())?;
            let channel_index = String::from_utf8(value.to_vec())?;
            if let Ok(Some(_channel_url)) = channel_tree.get(&channel_index) {
                println!("{} \t {}", channel_index, key_str);
            }
        }

        Ok(())
    }
}
