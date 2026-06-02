use crate::core::search::demo::search_demo;
use crate::types::music::Song;
use crate::types::search::{SearchRequest, SearchResult, SearchType};

pub fn search_all(request: &SearchRequest, builtin_changqing_enabled: bool) -> SearchResult {
    if request.search_type != SearchType::Song || request.keyword.trim().is_empty() {
        return empty_result(request.page);
    }

    match request.source.as_deref() {
        None | Some("all") => {
            let mut results = vec![search_demo(request)];
            if builtin_changqing_enabled {
                results.push(search_builtin_changqing(request));
            }
            merge_results(results, request.page, request.page_size)
        }
        Some("demo") => search_demo(request),
        Some("builtin:changqing-svip") => {
            if builtin_changqing_enabled {
                search_builtin_changqing(request)
            } else {
                empty_result(request.page)
            }
        }
        Some(_) => empty_result(request.page),
    }
}

fn search_builtin_changqing(request: &SearchRequest) -> SearchResult {
    let keyword = request.keyword.trim().to_lowercase();
    let songs = builtin_changqing_catalog()
        .into_iter()
        .filter(|song| {
            song.title.to_lowercase().contains(&keyword)
                || song
                    .artist
                    .as_deref()
                    .is_some_and(|artist| artist.to_lowercase().contains(&keyword))
                || song
                    .album
                    .as_deref()
                    .is_some_and(|album| album.to_lowercase().contains(&keyword))
                || song.source.to_lowercase().contains(&keyword)
                || keyword == "lx"
                || keyword == "长青"
        })
        .collect::<Vec<_>>();

    paginate(songs, request.page, request.page_size)
}

fn builtin_changqing_catalog() -> Vec<Song> {
    vec![
        Song {
            id: "builtin:changqing-svip:soundhelix-3".into(),
            source: "builtin:changqing-svip".into(),
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
            source: "builtin:changqing-svip".into(),
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

fn merge_results(results: Vec<SearchResult>, page: u32, page_size: u32) -> SearchResult {
    let songs = results
        .into_iter()
        .flat_map(|result| result.songs)
        .collect::<Vec<_>>();
    paginate(songs, page, page_size)
}

fn paginate(songs: Vec<Song>, page: u32, page_size: u32) -> SearchResult {
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
        total,
        page,
        has_more: end < songs.len(),
    }
}

fn empty_result(page: u32) -> SearchResult {
    SearchResult {
        songs: Vec::new(),
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
        assert!(result.songs.iter().any(|song| song.source == "demo"));
    }

    #[test]
    fn builtin_changqing_is_source_scoped_and_playable() {
        let result = search_all(&SearchRequest {
            keyword: "lx".into(),
            search_type: SearchType::Song,
            source: Some("builtin:changqing-svip".into()),
            page: 1,
            page_size: 20,
        }, true);

        assert_eq!(result.songs.len(), 2);
        assert!(result.songs.iter().all(|song| song.source == "builtin:changqing-svip"));
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
            .all(|song| song.source != "builtin:changqing-svip"));
    }
}
