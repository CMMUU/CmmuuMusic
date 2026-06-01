use rusqlite::Connection;

use super::DbError;

/// 当前 schema 版本。每次变更 schema 时递增并追加迁移步骤。
const SCHEMA_VERSION: i64 = 1;

/// 按需执行迁移。使用 `PRAGMA user_version` 记录已应用的版本。
pub fn run(conn: &Connection) -> Result<(), DbError> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;

    if current < 1 {
        conn.execute_batch(V1)
            .map_err(|e| DbError::Migration(format!("应用 v1 失败: {e}")))?;
    }

    conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    Ok(())
}

/// v1 初始 schema，对齐 SDD §8.1。
const V1: &str = r#"
CREATE TABLE IF NOT EXISTS playlists (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    cover_url   TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS songs (
    id          TEXT PRIMARY KEY,
    source      TEXT NOT NULL,
    title       TEXT NOT NULL,
    artist      TEXT,
    album       TEXT,
    cover_url   TEXT,
    duration    REAL,
    lyric_text  TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS playlist_songs (
    playlist_id TEXT NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    song_id     TEXT NOT NULL REFERENCES songs(id),
    sort_order  INTEGER NOT NULL,
    added_at    TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (playlist_id, song_id)
);

CREATE TABLE IF NOT EXISTS play_history (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    song_id         TEXT NOT NULL REFERENCES songs(id),
    played_at       TEXT NOT NULL DEFAULT (datetime('now')),
    duration_played REAL
);

CREATE TABLE IF NOT EXISTS plugins (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    version      TEXT NOT NULL,
    author       TEXT,
    plugin_type  TEXT NOT NULL,
    file_path    TEXT NOT NULL,
    enabled      INTEGER NOT NULL DEFAULT 1,
    installed_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cache_index (
    url_hash     TEXT PRIMARY KEY,
    original_url TEXT NOT NULL,
    local_path   TEXT NOT NULL,
    file_size    INTEGER,
    expires_at   TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_play_history_played_at ON play_history(played_at);
CREATE INDEX IF NOT EXISTS idx_cache_expires ON cache_index(expires_at);
CREATE INDEX IF NOT EXISTS idx_playlist_songs_order ON playlist_songs(playlist_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_songs_source ON songs(source);
"#;
