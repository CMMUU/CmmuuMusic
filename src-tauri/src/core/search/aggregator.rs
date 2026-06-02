use crate::core::search::demo::search_demo;
use crate::types::music::Song;
use crate::types::search::{SearchRequest, SearchResult, SearchType, SourcePlaylist};

const BUILTIN_CHANGQING_SOURCE: &str = "builtin:changqing-svip";

pub fn search_all(request: &SearchRequest, builtin_changqing_enabled: bool) -> SearchResult {
    if request.keyword.trim().is_empty() {
        return empty_result(request.page);
    }

    match request.search_type {
        SearchType::Song => search_songs(request, builtin_changqing_enabled),
        SearchType::Playlist => search_playlists(request, builtin_changqing_enabled),
        SearchType::Album | SearchType::Artist => empty_result(request.page),
    }
}

pub fn list_source_playlist_songs(
    source: &str,
    playlist_id: &str,
    builtin_changqing_enabled: bool,
) -> Vec<Song> {
    if source != BUILTIN_CHANGQING_SOURCE || !builtin_changqing_enabled {
        return Vec::new();
    }

    match playlist_id {
        "builtin:changqing-svip:playlist:featured" => builtin_changqing_catalog(),
        "builtin:changqing-svip:playlist:lx" => builtin_changqing_catalog()
            .into_iter()
            .filter(|song| song.album.as_deref() == Some("LX 兼容资源占位"))
            .collect(),
        "builtin:changqing-svip:playlist:playable" => builtin_changqing_catalog()
            .into_iter()
            .filter(|song| song.play_url.is_some())
            .collect(),
        _ => Vec::new(),
    }
}

fn search_songs(request: &SearchRequest, builtin_changqing_enabled: bool) -> SearchResult {
    match request.source.as_deref() {
        None | Some("all") => {
            let mut results = vec![search_demo(request)];
            if builtin_changqing_enabled {
                results.push(search_builtin_changqing(request));
            }
            merge_song_results(results, request.page, request.page_size)
        }
        Some("demo") => search_demo(request),
        Some(BUILTIN_CHANGQING_SOURCE) => {
            if builtin_changqing_enabled {
                search_builtin_changqing(request)
            } else {
                empty_result(request.page)
            }
        }
        Some(_) => empty_result(request.page),
    }
}

fn search_playlists(request: &SearchRequest, builtin_changqing_enabled: bool) -> SearchResult {
    match request.source.as_deref() {
        None | Some("all") | Some(BUILTIN_CHANGQING_SOURCE) if builtin_changqing_enabled => {
            search_builtin_changqing_playlists(request)
        }
        _ => empty_result(request.page),
    }
}

fn search_builtin_changqing(request: &SearchRequest) -> SearchResult {
    let keyword = request.keyword.trim().to_lowercase();
    let songs = builtin_changqing_catalog()
        .into_iter()
        .filter(|song| matches_song(song, &keyword))
        .collect::<Vec<_>>();

    paginate_songs(songs, request.page, request.page_size)
}

fn search_builtin_changqing_playlists(request: &SearchRequest) -> SearchResult {
    let keyword = request.keyword.trim().to_lowercase();
    let playlists = builtin_changqing_playlists()
        .into_iter()
        .filter(|playlist| matches_playlist(playlist, &keyword))
        .collect::<Vec<_>>();

    paginate_playlists(playlists, request.page, request.page_size)
}

fn matches_song(song: &Song, keyword: &str) -> bool {
    song.title.to_lowercase().contains(keyword)
        || song
            .artist
            .as_deref()
            .is_some_and(|artist| artist.to_lowercase().contains(keyword))
        || song
            .album
            .as_deref()
            .is_some_and(|album| album.to_lowercase().contains(keyword))
        || song.source.to_lowercase().contains(keyword)
        || keyword == "lx"
        || keyword == "长青"
}

fn matches_playlist(playlist: &SourcePlaylist, keyword: &str) -> bool {
    playlist.name.to_lowercase().contains(keyword)
        || playlist.source.to_lowercase().contains(keyword)
        || playlist
            .description
            .as_deref()
            .is_some_and(|description| description.to_lowercase().contains(keyword))
        || keyword == "lx"
        || keyword == "长青"
}

fn builtin_changqing_catalog() -> Vec<Song> {
    vec![
        Song {
            id: "builtin:changqing-svip:soundhelix-3".into(),
            source: BUILTIN_CHANGQING_SOURCE.into(),
            title: "长青 SVIP 测试曲 1".into(),
            artist: Some("SoundHelix".into()),
            album: Some("受控内置音源".into()),
            cover_url: None,
            duration: Some(342.0),
            lyric_text: Some("受控内置 provider 可播放测试曲。".into()),
            play_url: Some("https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3".into()),
        },
        Song {
            id: "builtin:changqing-svip:soundhelix-4".into(),
            source: BUILTIN_CHANGQING_SOURCE.into(),
            title: "长青 SVIP 测试曲 2".into(),
            artist: Some("SoundHelix".into()),
            album: Some("LX 兼容资源占位".into()),
            cover_url: None,
            duration: Some(311.0),
            lyric_text: Some("当前不执行第三方脚本，仅使用内置 provider 返回播放地址。".into()),
            play_url: Some("https://www.soundhelix.com/examples/mp3/SoundHelix-Song-4.mp3".into()),
        },
    ]
}

fn builtin_changqing_playlists() -> Vec<SourcePlaylist> {
    vec![
        SourcePlaylist {
            id: "builtin:changqing-svip:playlist:featured".into(),
            source: BUILTIN_CHANGQING_SOURCE.into(),
            name: "长青 SVIP 精选".into(),
            description: Some("受控内置音源返回的精选测试歌单。".into()),
            cover_url: None,
            song_count: Some(2),
        },
        SourcePlaylist {
            id: "builtin:changqing-svip:playlist:lx".into(),
            source: BUILTIN_CHANGQING_SOURCE.into(),
            name: "LX 兼容资源测试".into(),
            description: Some("用于验证 LX 兼容音源歌单和歌曲加载链路。".into()),
            cover_url: None,
            song_count: Some(1),
        },
        SourcePlaylist {
            id: "builtin:changqing-svip:playlist:playable".into(),
            source: BUILTIN_CHANGQING_SOURCE.into(),
            name: "可播放直链集合".into(),
            description: Some("只包含受控 HTTPS 播放地址的歌曲。".into()),
            cover_url: None,
            song_count: Some(2),
        },
    ]
}

fn merge_song_results(results: Vec<SearchResult>, page: u32, page_size: u32) -> SearchResult {
    let songs = results
        .into_iter()
        .flat_map(|result| result.songs)
        .collect::<Vec<_>>();
    paginate_songs(songs, page, page_size)
}

fn paginate_songs(songs: Vec<Song>, page: u32, page_size: u32) -> SearchResult {
    let page = page.max(1);
    let page_size = page_size.max(1);
    let total = songs.len() as u32;
    let start = ((page - 1) * page_size) as usize;
    let end = start.saturating_add(page_size as usize).min(songs.len());
    let page_songs = if start < songs.len() {
        songs[start..end].to_vec()
    } else {
        Vec::new()
    };

    SearchResult {
        songs: page_songs,
        playlists: Vec::new(),
        total,
        page,
        has_more: end < songs.len(),
    }
}

fn paginate_playlists(playlists: Vec<SourcePlaylist>, page: u32, page_size: u32) -> SearchResult {
    let page = page.max(1);
    let page_size = page_size.max(1);
    let total = playlists.len() as u32;
    let start = ((page - 1) * page_size) as usize;
    let end = start.saturating_add(page_size as usize).min(playlists.len());
    let page_playlists = if start < playlists.len() {
        playlists[start..end].to_vec()
    } else {
        Vec::new()
    };

    SearchResult {
        songs: Vec::new(),
        playlists: page_playlists,
        total,
        page,
        has_more: end < playlists.len(),
    }
}

fn empty_result(page: u32) -> SearchResult {
    SearchResult {
        songs: Vec::new(),
        playlists: Vec::new(),
        total: 0,
        page: page.max(1),
        has_more: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_search_includes_demo() {
        let result = search_all(&SearchRequest {
            keyword: "soundhelix".into(),
            search_type: SearchType::Song,
            source: None,
            page: 1,
            page_size: 20,
        }, true);

        assert!(!result.songs.is_empty());
        assert!(result.playlists.is_empty());
        assert!(result.songs.iter().any(|song| song.source == "demo"));
    }

    #[test]
    fn builtin_changqing_is_source_scoped_and_playable() {
        let result = search_all(&SearchRequest {
            keyword: "lx".into(),
            search_type: SearchType::Song,
            source: Some(BUILTIN_CHANGQING_SOURCE.into()),
            page: 1,
            page_size: 20,
        }, true);

        assert_eq!(result.songs.len(), 2);
        assert!(result.playlists.is_empty());
        assert!(result.songs.iter().all(|song| song.source == BUILTIN_CHANGQING_SOURCE));
        assert!(result.songs.iter().all(|song| song.play_url.is_some()));
    }

    #[test]
    fn all_search_excludes_builtin_when_disabled() {
        let result = search_all(&SearchRequest {
            keyword: "lx".into(),
            search_type: SearchType::Song,
            source: None,
            page: 1,
            page_size: 20,
        }, false);

        assert!(result
            .songs
            .iter()
            .all(|song| song.source != BUILTIN_CHANGQING_SOURCE));
    }

    #[test]
    fn playlist_search_returns_builtin_when_enabled() {
        let result = search_all(&SearchRequest {
            keyword: "长青".into(),
            search_type: SearchType::Playlist,
            source: Some(BUILTIN_CHANGQING_SOURCE.into()),
            page: 1,
            page_size: 20,
        }, true);

        assert!(result.songs.is_empty());
        assert!(!result.playlists.is_empty());
        assert!(result
            .playlists
            .iter()
            .all(|playlist| playlist.source == BUILTIN_CHANGQING_SOURCE));
    }

    #[test]
    fn playlist_search_excludes_builtin_when_disabled() {
        let result = search_all(&SearchRequest {
            keyword: "长青".into(),
            search_type: SearchType::Playlist,
            source: Some(BUILTIN_CHANGQING_SOURCE.into()),
            page: 1,
            page_size: 20,
        }, false);

        assert!(result.playlists.is_empty());
    }

    #[test]
    fn all_playlist_search_includes_enabled_builtin() {
        let result = search_all(&SearchRequest {
            keyword: "lx".into(),
            search_type: SearchType::Playlist,
            source: None,
            page: 1,
            page_size: 20,
        }, true);

        assert!(!result.playlists.is_empty());
    }

    #[test]
    fn list_source_playlist_songs_returns_playable_songs() {
        let songs = list_source_playlist_songs(
            BUILTIN_CHANGQING_SOURCE,
            "builtin:changqing-svip:playlist:featured",
            true,
        );

        assert!(!songs.is_empty());
        assert!(songs.iter().all(|song| song.source == BUILTIN_CHANGQING_SOURCE));
        assert!(songs.iter().all(|song| song.play_url.is_some()));
    }

    #[test]
    fn list_source_playlist_songs_returns_empty_when_disabled() {
        let songs = list_source_playlist_songs(
            BUILTIN_CHANGQING_SOURCE,
            "builtin:changqing-svip:playlist:featured",
            false,
        );

        assert!(songs.is_empty());
    }
}
