use crate::core::search::demo::search_demo;
use crate::types::music::Song;
use crate::types::search::{SearchRequest, SearchResult, SearchType};

pub fn search_all(request: &SearchRequest) -> SearchResult {
    if request.search_type != SearchType::Song || request.keyword.trim().is_empty() {
        return empty_result(request.page);
    }

    match request.source.as_deref() {
        None | Some("all") => merge_results(vec![search_demo(request)], request.page, request.page_size),
        Some("demo") => search_demo(request),
        Some("builtin:changqing-svip") => builtin_placeholder(request),
        Some(_) => empty_result(request.page),
    }
}

fn builtin_placeholder(request: &SearchRequest) -> SearchResult {
    let keyword = request.keyword.trim();
    let songs = if "长青SVIP音源".contains(keyword) || keyword.eq_ignore_ascii_case("lx") {
        vec![Song {
            id: "builtin:changqing-svip:placeholder".into(),
            source: "builtin:changqing-svip".into(),
            title: "长青SVIP音源已内置".into(),
            artist: Some("LX 兼容音源".into()),
            album: Some("等待沙箱运行时接入".into()),
            cover_url: None,
            duration: None,
            lyric_text: Some("该音源已作为资源内置，真实搜索将在 LX 兼容运行时阶段启用。".into()),
            play_url: None,
        }]
    } else {
        Vec::new()
    };

    paginate(songs, request.page, request.page_size)
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
        });

        assert!(!result.songs.is_empty());
        assert!(result.songs.iter().any(|song| song.source == "demo"));
    }

    #[test]
    fn builtin_placeholder_is_source_scoped() {
        let result = search_all(&SearchRequest {
            keyword: "lx".into(),
            search_type: SearchType::Song,
            source: Some("builtin:changqing-svip".into()),
            page: 1,
            page_size: 20,
        });

        assert_eq!(result.songs.len(), 1);
        assert_eq!(result.songs[0].source, "builtin:changqing-svip");
    }
}
