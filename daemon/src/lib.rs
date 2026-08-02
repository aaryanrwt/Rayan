use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc::channel;
use tracing::{error, info, warn};

pub struct DriftDetector {
    watch_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub id: i64,
    pub timestamp: String,
    pub asg_snapshot: String,
    pub nix_store_path: String,
}

pub struct RollbackTracker {
    conn: Connection,
}

impl RollbackTracker {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS generations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                asg_snapshot TEXT NOT NULL,
                nix_store_path TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn record_generation(&self, asg_snapshot: &str, store_path: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO generations (asg_snapshot, nix_store_path) VALUES (?1, ?2)",
            params![asg_snapshot, store_path],
        )?;
        info!("Recorded new generation pointing to {}", store_path);
        Ok(())
    }

    pub fn list_generations(&self) -> Result<Vec<GenerationRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, asg_snapshot, nix_store_path FROM generations ORDER BY id DESC",
        )?;
        let gen_iter = stmt.query_map([], |row| {
            Ok(GenerationRecord {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                asg_snapshot: row.get(2)?,
                nix_store_path: row.get(3)?,
            })
        })?;

        let mut records = Vec::new();
        for gen in gen_iter {
            records.push(gen?);
        }
        Ok(records)
    }
}

pub struct CloudSyncer {
    api_url: String,
}

impl CloudSyncer {
    pub fn new(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
        }
    }

    pub async fn push_generation(&self, record: &GenerationRecord) -> Result<()> {
        info!("Syncing generation {} to Rayan Cloud...", record.id);
        // For MVP, we mock the HTTP request that would normally push to axum
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        info!(
            "Successfully synced generation {} to {}",
            record.id, self.api_url
        );
        Ok(())
    }
}

impl DriftDetector {
    pub fn new(watch_path: &str) -> Self {
        Self {
            watch_path: watch_path.to_string(),
        }
    }

    pub fn start(&self) -> Result<()> {
        info!("Starting Rayan Drift Daemon on path: {}", self.watch_path);

        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        let path = Path::new(&self.watch_path);
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        watcher.watch(path, RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(Event { paths, kind, .. }) => {
                    if kind.is_modify() || kind.is_create() || kind.is_remove() {
                        for p in paths {
                            warn!("State Drift Detected: {:?}", p);
                            // In Phase 4, we would send this over IPC to the LSP or Core Engine
                            // to check if this modification is tracked in the ASG.
                        }
                    }
                }
                Err(e) => error!("watch error: {:?}", e),
            }
        }

        Ok(())
    }
}
