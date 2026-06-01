use rusqlite::{params, OptionalExtension};

use super::{Database, DbError};
use crate::types::playlist::Playlist;

impl Database {
    /// 创建播放列表。
    pub fn create_playlist(&self, id: &str, name: &str) -> Result<Playlist, DbError> {
        self.with_conn(|c| {
            c.execute(
                "INSERT INTO playlists (id, name) VALUES (?1, ?2)",
                params![id, name],
            )?;
            c.query_row(
                "SELECT id, name, description, cover_url, created_at, updated_at
                 FROM playlists WHERE id = ?1",
                params![id],
                row_to_playlist,
            )
        })
    }

    /// 列出所有播放列表（不含歌曲明细）。
    pub fn list_playlists(&self) -> Result<Vec<Playlist>, DbError> {
        self.with_conn(|c| {
            let mut stmt = c.prepare(
                "SELECT id, name, description, cover_url, created_at, updated_at
                 FROM playlists ORDER BY updated_at DESC",
            )?;
            let rows = stmt
                .query_map([], row_to_playlist)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    /// 删除播放列表。
    pub fn delete_playlist(&self, id: &str) -> Result<bool, DbError> {
        self.with_conn(|c| {
            let n = c.execute("DELETE FROM playlists WHERE id = ?1", params![id])?;
            Ok(n > 0)
        })
    }

    /// 读取设置项。
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, DbError> {
        self.with_conn(|c| {
            c.query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |r| r.get::<_, String>(0),
            )
            .optional()
        })
    }

    /// 写入设置项（upsert）。
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), DbError> {
        self.with_conn(|c| {
            c.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![key, value],
            )?;
            Ok(())
        })
    }
}

fn row_to_playlist(row: &rusqlite::Row<'_>) -> rusqlite::Result<Playlist> {
    Ok(Playlist {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        cover_url: row.get(3)?,
        songs: Vec::new(),
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Database {
        let db = Database::open_in_memory().unwrap();
        db.run_migrations().unwrap();
        db
    }

    #[test]
    fn playlist_crud() {
        let db = setup();
        let pl = db.create_playlist("p1", "我的歌单").unwrap();
        assert_eq!(pl.name, "我的歌单");

        let list = db.list_playlists().unwrap();
        assert_eq!(list.len(), 1);

        assert!(db.delete_playlist("p1").unwrap());
        assert!(db.list_playlists().unwrap().is_empty());
    }

    #[test]
    fn settings_upsert() {
        let db = setup();
        assert_eq!(db.get_setting("volume").unwrap(), None);
        db.set_setting("volume", "0.8").unwrap();
        assert_eq!(db.get_setting("volume").unwrap(), Some("0.8".into()));
        db.set_setting("volume", "0.5").unwrap();
        assert_eq!(db.get_setting("volume").unwrap(), Some("0.5".into()));
    }
}
