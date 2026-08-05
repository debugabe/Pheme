use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::similarity::cosine_similarity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMemory {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub topics: Vec<String>,
    pub published_at: DateTime<Utc>,
    pub embedding: Vec<f32>,
}

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    pub fn open(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)
            .with_context(|| format!("Falha ao abrir banco SQLite em {:?}", db_path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS episode_memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                summary TEXT NOT NULL,
                topics TEXT NOT NULL,
                published_at TEXT NOT NULL,
                embedding_json TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn save_episode(
        &self,
        title: &str,
        summary: &str,
        topics: &[String],
        published_at: DateTime<Utc>,
        embedding: &[f32],
    ) -> Result<i64> {
        let topics_json = serde_json::to_string(topics)?;
        let embedding_json = serde_json::to_string(embedding)?;
        let date_str = published_at.to_rfc3339();

        self.conn.execute(
            "INSERT INTO episode_memories (title, summary, topics, published_at, embedding_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![title, summary, topics_json, date_str, embedding_json],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn find_similar_episodes(
        &self,
        target_embedding: &[f32],
        min_threshold: f32,
        limit: usize,
    ) -> Result<Vec<(EpisodeMemory, f32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, summary, topics, published_at, embedding_json FROM episode_memories",
        )?;

        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let summary: String = row.get(2)?;
            let topics_json: String = row.get(3)?;
            let date_str: String = row.get(4)?;
            let embedding_json: String = row.get(5)?;

            Ok((id, title, summary, topics_json, date_str, embedding_json))
        })?;

        let mut results = Vec::new();
        for row_res in rows {
            let (id, title, summary, topics_json, date_str, embedding_json) = row_res?;

            let topics: Vec<String> = serde_json::from_str(&topics_json).unwrap_or_default();
            let embedding: Vec<f32> = serde_json::from_str(&embedding_json).unwrap_or_default();
            let published_at = DateTime::parse_from_rfc3339(&date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let sim = cosine_similarity(target_embedding, &embedding);
            if sim >= min_threshold {
                let ep = EpisodeMemory {
                    id,
                    title,
                    summary,
                    topics,
                    published_at,
                    embedding,
                };
                results.push((ep, sim));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }
}
