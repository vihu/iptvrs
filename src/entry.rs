use serde::Serialize;

#[derive(Debug, sqlx::FromRow)]
pub struct IptvEntry {
    pub channel_index: u32,
    pub channel_name: String,
    pub channel_url: String,
    pub playlist_name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct PublicIptvEntry {
    pub channel_index: u32,
    pub channel_name: String,
    pub playlist_name: String,
}
