use rusqlite::{params, OptionalExtension};

use super::{Database, DbError};
use crate::types::music::Song;
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
}
