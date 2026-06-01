use rusqlite::{params, OptionalExtension};

use super::{Database, DbError};
use crate::types::music::{PlayHistoryRecord, Song};
use crate::types::playlist::Playlist;
use crate::types::plugin::{PluginInfo, PluginRecord, PluginStatus, PluginType};

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

    pub fn upsert_song(&self, song: &Song) -> Result<(), DbError> {
        self.with_conn(|c| upsert_song_conn(c, song)).map(|_| ())
    }

    pub fn add_song_to_playlist(&self, playlist_id: &str, song: &Song) -> Result<(), DbError> {
        self.with_conn(|c| {
            upsert_song_conn(c, song)?;
            let next_order: i64 = c.query_row(
                "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM playlist_songs WHERE playlist_id = ?1",
                params![playlist_id],
                |r| r.get(0),
            )?;
            c.execute(
                "INSERT OR IGNORE INTO playlist_songs (playlist_id, song_id, sort_order)
                 VALUES (?1, ?2, ?3)",
                params![playlist_id, song.id, next_order],
            )?;
            Ok(())
        })
    }

    pub fn list_playlist_songs(&self, playlist_id: &str) -> Result<Vec<Song>, DbError> {
        self.with_conn(|c| {
            let mut stmt = c.prepare(
                "SELECT s.id, s.source, s.title, s.artist, s.album, s.cover_url, s.duration, s.lyric_text
                 FROM playlist_songs ps
                 JOIN songs s ON s.id = ps.song_id
                 WHERE ps.playlist_id = ?1
                 ORDER BY ps.sort_order ASC, ps.added_at ASC",
            )?;
            let rows = stmt
                .query_map(params![playlist_id], row_to_song)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
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

    pub fn record_play_history(
        &self,
        song: &Song,
        duration_played: Option<f64>,
    ) -> Result<PlayHistoryRecord, DbError> {
        self.with_conn(|c| {
            upsert_song_conn(c, song)?;
            c.execute(
                "INSERT INTO play_history (song_id, duration_played) VALUES (?1, ?2)",
                params![song.id, duration_played],
            )?;
            let id = c.last_insert_rowid();
            c.query_row(
                "SELECT ph.id, s.id, s.source, s.title, s.artist, s.album, s.cover_url, s.duration, s.lyric_text,
                        ph.played_at, ph.duration_played
                 FROM play_history ph
                 JOIN songs s ON s.id = ph.song_id
                 WHERE ph.id = ?1",
                params![id],
                row_to_play_history,
            )
        })
    }

    pub fn list_play_history(&self, limit: u32) -> Result<Vec<PlayHistoryRecord>, DbError> {
        self.with_conn(|c| {
            let mut stmt = c.prepare(
                "SELECT ph.id, s.id, s.source, s.title, s.artist, s.album, s.cover_url, s.duration, s.lyric_text,
                        ph.played_at, ph.duration_played
                 FROM play_history ph
                 JOIN songs s ON s.id = ph.song_id
                 ORDER BY ph.played_at DESC, ph.id DESC
                 LIMIT ?1",
            )?;
            let rows = stmt
                .query_map(params![limit.max(1)], row_to_play_history)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    pub fn list_recent_songs(&self, limit: u32) -> Result<Vec<Song>, DbError> {
        self.with_conn(|c| {
            let mut stmt = c.prepare(
                "SELECT s.id, s.source, s.title, s.artist, s.album, s.cover_url, s.duration, s.lyric_text
                 FROM songs s
                 JOIN (
                    SELECT song_id, MAX(id) AS last_history_id
                    FROM play_history
                    GROUP BY song_id
                 ) recent ON recent.song_id = s.id
                 JOIN play_history ph ON ph.id = recent.last_history_id
                 ORDER BY ph.played_at DESC, ph.id DESC
                 LIMIT ?1",
            )?;
            let rows = stmt
                .query_map(params![limit.max(1)], row_to_song)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    pub fn set_song_lyrics(&self, song_id: &str, lyric_text: &str) -> Result<bool, DbError> {
        self.with_conn(|c| {
            let updated = c.execute(
                "UPDATE songs SET lyric_text = ?2 WHERE id = ?1",
                params![song_id, lyric_text],
            )?;
            Ok(updated > 0)
        })
    }

    pub fn get_song_lyrics(&self, song_id: &str) -> Result<Option<String>, DbError> {
        self.with_conn(|c| {
            c.query_row(
                "SELECT lyric_text FROM songs WHERE id = ?1",
                params![song_id],
                |r| r.get::<_, Option<String>>(0),
            )
            .optional()
            .map(|value| value.flatten())
        })
    }

    pub fn upsert_plugin_record(&self, plugin: &PluginRecord) -> Result<(), DbError> {
        self.with_conn(|c| {
            c.execute(
                "INSERT INTO plugins (id, name, version, author, plugin_type, file_path, enabled, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))
                 ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    version = excluded.version,
                    author = excluded.author,
                    plugin_type = excluded.plugin_type,
                    file_path = excluded.file_path,
                    enabled = excluded.enabled,
                    updated_at = datetime('now')",
                params![
                    plugin.info.id,
                    plugin.info.name,
                    plugin.info.version,
                    plugin.info.author,
                    plugin.info.plugin_type.as_str(),
                    plugin.file_path,
                    plugin.enabled as i32,
                ],
            )?;
            Ok(())
        })
    }

    pub fn list_plugin_records(&self) -> Result<Vec<PluginRecord>, DbError> {
        self.with_conn(|c| {
            let mut stmt = c.prepare(
                "SELECT id, name, version, author, plugin_type, file_path, enabled, installed_at, updated_at
                 FROM plugins ORDER BY updated_at DESC",
            )?;
            let rows = stmt
                .query_map([], row_to_plugin_record)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    pub fn set_plugin_enabled(&self, id: &str, enabled: bool) -> Result<bool, DbError> {
        self.with_conn(|c| {
            let updated = c.execute(
                "UPDATE plugins SET enabled = ?2, updated_at = datetime('now') WHERE id = ?1",
                params![id, enabled as i32],
            )?;
            Ok(updated > 0)
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

fn row_to_song(row: &rusqlite::Row<'_>) -> rusqlite::Result<Song> {
    Ok(Song {
        id: row.get(0)?,
        source: row.get(1)?,
        title: row.get(2)?,
        artist: row.get(3)?,
        album: row.get(4)?,
        cover_url: row.get(5)?,
        duration: row.get(6)?,
        lyric_text: row.get(7)?,
        play_url: None,
    })
}

fn row_to_play_history(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlayHistoryRecord> {
    Ok(PlayHistoryRecord {
        id: row.get(0)?,
        song: Song {
            id: row.get(1)?,
            source: row.get(2)?,
            title: row.get(3)?,
            artist: row.get(4)?,
            album: row.get(5)?,
            cover_url: row.get(6)?,
            duration: row.get(7)?,
            lyric_text: row.get(8)?,
            play_url: None,
        },
        played_at: row.get(9)?,
        duration_played: row.get(10)?,
    })
}

fn row_to_plugin_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<PluginRecord> {
    let id: String = row.get(0)?;
    let name: String = row.get(1)?;
    let version: String = row.get(2)?;
    let author: Option<String> = row.get(3)?;
    let plugin_type: String = row.get(4)?;
    let file_path: String = row.get(5)?;
    let enabled: i32 = row.get(6)?;
    let installed_at: String = row.get(7)?;
    let updated_at: String = row.get(8)?;

    Ok(PluginRecord {
        info: PluginInfo {
            id,
            name,
            description: String::new(),
            version,
            author: author.unwrap_or_default(),
            homepage: String::new(),
            plugin_type: PluginType::from_str(&plugin_type),
        },
        sources: Vec::new(),
        file_path,
        enabled: enabled != 0,
        status: if enabled != 0 {
            PluginStatus::Ready
        } else {
            PluginStatus::Disabled
        },
        installed_at,
        updated_at,
    })
}

fn upsert_song_conn(c: &rusqlite::Connection, song: &Song) -> rusqlite::Result<usize> {
    c.execute(
        "INSERT INTO songs (id, source, title, artist, album, cover_url, duration, lyric_text)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET
            source = excluded.source,
            title = excluded.title,
            artist = excluded.artist,
            album = excluded.album,
            cover_url = excluded.cover_url,
            duration = excluded.duration,
            lyric_text = excluded.lyric_text",
        params![
            song.id,
            song.source,
            song.title,
            song.artist,
            song.album,
            song.cover_url,
            song.duration,
            song.lyric_text
        ],
    )
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

    fn sample_song() -> Song {
        Song {
            id: "demo:1".into(),
            source: "demo".into(),
            title: "Demo Song".into(),
            artist: Some("Demo Artist".into()),
            album: Some("Demo Album".into()),
            cover_url: None,
            duration: Some(12.0),
            lyric_text: None,
            play_url: Some("https://example.com/demo.mp3".into()),
        }
    }

    #[test]
    fn playlist_song_crud() {
        let db = setup();
        let pl = db.create_playlist("p1", "我的歌单").unwrap();
        let song = sample_song();

        db.add_song_to_playlist(&pl.id, &song).unwrap();
        db.add_song_to_playlist(&pl.id, &song).unwrap();

        let songs = db.list_playlist_songs(&pl.id).unwrap();
        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].id, song.id);
        assert_eq!(songs[0].play_url, None);
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

    #[test]
    fn play_history_and_recent_songs() {
        let db = setup();
        let song = sample_song();

        let history = db.record_play_history(&song, Some(8.0)).unwrap();
        assert_eq!(history.song.id, song.id);
        assert_eq!(history.duration_played, Some(8.0));

        db.record_play_history(&song, Some(9.0)).unwrap();
        let all = db.list_play_history(10).unwrap();
        assert_eq!(all.len(), 2);

        let recent = db.list_recent_songs(10).unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, song.id);
    }

    #[test]
    fn song_lyrics_cache() {
        let db = setup();
        let song = sample_song();
        db.upsert_song(&song).unwrap();

        assert_eq!(db.get_song_lyrics(&song.id).unwrap(), None);
        assert!(db.set_song_lyrics(&song.id, "[00:00]Demo").unwrap());
        assert_eq!(
            db.get_song_lyrics(&song.id).unwrap(),
            Some("[00:00]Demo".into())
        );
        assert!(!db.set_song_lyrics("missing", "lyric").unwrap());
    }

    #[test]
    fn plugin_record_crud() {
        let db = setup();
        let plugin = PluginRecord {
            info: PluginInfo {
                id: "plugin:demo".into(),
                name: "Demo Plugin".into(),
                description: String::new(),
                version: "1.0.0".into(),
                author: "tester".into(),
                homepage: String::new(),
                plugin_type: PluginType::Lx,
            },
            sources: Vec::new(),
            file_path: "resources/demo.js".into(),
            enabled: true,
            status: PluginStatus::Ready,
            installed_at: String::new(),
            updated_at: String::new(),
        };

        db.upsert_plugin_record(&plugin).unwrap();
        assert!(db.set_plugin_enabled(&plugin.info.id, false).unwrap());
        let plugins = db.list_plugin_records().unwrap();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].info.id, plugin.info.id);
        assert_eq!(plugins[0].info.plugin_type, PluginType::Lx);
        assert!(!plugins[0].enabled);
    }
}
