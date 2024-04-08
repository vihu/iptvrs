use crate::{
    entry::{IptvEntry, PublicIptvEntry},
    settings::Settings,
};
use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool};
use std::str::FromStr;

pub struct DB {
    pub pool: SqlitePool,
}

impl DB {
    pub async fn from_settings(settings: &Settings) -> Result<Self> {
        let conn = SqliteConnectOptions::from_str(&settings.db_url)?
            .journal_mode(SqliteJournalMode::Wal)
            .read_only(false)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(conn).await?;

        Ok(DB { pool })
    }

    pub async fn insert_or_update_entries(&self, entries: Vec<IptvEntry>) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let upsert_sql = r#"
            INSERT INTO iptvrs (
                channel_index,
                channel_url,
                channel_name,
                playlist_name
            )
            VALUES (?, ?, ?, ?)
            ON CONFLICT(channel_index) DO UPDATE SET
                channel_url = excluded.channel_url,
                channel_name = excluded.channel_name,
                playlist_name = excluded.playlist_name;
            "#;

        for entry in &entries {
            sqlx::query(upsert_sql)
                .bind(entry.channel_index as i32)
                .bind(&entry.channel_url)
                .bind(&entry.channel_name)
                .bind(&entry.playlist_name)
                .execute(&mut *tx)
                .await?;

            // Check if the entry exists in the FTS table
            let exists =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM iptvrs_fts WHERE rowid = ?")
                    .bind(entry.channel_index as i32)
                    .fetch_one(&mut *tx)
                    .await?;

            if exists > 0 {
                // Update the existing FTS entry
                sqlx::query(
                    "UPDATE iptvrs_fts SET channel_name = ?, playlist_name = ? WHERE rowid = ?",
                )
                .bind(&entry.channel_name)
                .bind(&entry.playlist_name)
                .bind(entry.channel_index as i32)
                .execute(&mut *tx)
                .await?;
            } else {
                // Insert new FTS entry
                sqlx::query(
                    "INSERT INTO iptvrs_fts (rowid, channel_name, playlist_name) VALUES (?, ?, ?)",
                )
                .bind(entry.channel_index as i32)
                .bind(&entry.channel_name)
                .bind(&entry.playlist_name)
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn list_entries(&self) -> Result<Vec<PublicIptvEntry>> {
        let query = "SELECT channel_index, channel_name, playlist_name FROM iptvrs";
        let result = sqlx::query_as::<_, PublicIptvEntry>(query)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    pub async fn search_entries(&self, search_term: &str) -> Result<Vec<PublicIptvEntry>> {
        let query = r#"
            SELECT channel_index, channel_name, playlist_name
            FROM iptvrs
            WHERE channel_index IN (
                SELECT rowid
                FROM iptvrs_fts
                WHERE iptvrs_fts MATCH ?
            )
        "#;
        let entries = sqlx::query_as::<_, PublicIptvEntry>(query)
            .bind(format!("{}*", search_term)) // Use * for prefix search in FTS
            .fetch_all(&self.pool)
            .await?;

        Ok(entries)
    }

    pub async fn get_entry_by_channel_index(
        &self,
        channel_index: usize,
    ) -> Result<Option<IptvEntry>> {
        let query = r#"SELECT * FROM iptvrs where channel_index = $1"#;
        let result = sqlx::query_as::<_, IptvEntry>(query)
            .bind(channel_index as i32)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }
}
