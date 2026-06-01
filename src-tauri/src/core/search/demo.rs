use crate::types::music::Song;
use crate::types::search::{SearchRequest, SearchResult, SearchType};

const DEMO_AUDIO_URL: &str = "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3";

pub fn search_demo(request: &SearchRequest) -> SearchResult {
    if request.search_type != SearchType::Song || request.keyword.trim().is_empty() {
        return empty_result(request.page);
    }

    if request.source.as_deref().is_some_and(|source| source != "demo") {
        return empty_result(request.page);
    }

    let keyword = request.keyword.trim().to_lowercase();
    let matched: Vec<Song> = demo_catalog()
        .into_iter()
        .filter(|song| matches_song(song, &keyword))
        .collect();

    paginate(matched, request.page, request.page_size)
}

fn demo_catalog() -> Vec<Song> {
    vec![
        Song {
            id: "demo:soundhelix-1".into(),
            source: "demo".into(),
            title: "SoundHelix Song 1".into(),
            artist: Some("SoundHelix".into()),
            album: Some("Demo Catalog".into()),
            cover_url: None,
            duration: Some(372.0),
            lyric_text: None,
            play_url: Some(DEMO_AUDIO_URL.into()),
        },
        Song {
            id: "demo:soundhelix-2".into(),
            source: "demo".into(),
            title: "SoundHelix Song 2".into(),
            artist: Some("SoundHelix".into()),
            album: Some("Demo Catalog".into()),
            cover_url: None,
            duration: Some(312.0),
            lyric_text: None,
            play_url: Some("https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3".into()),
        },
    ]
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
    fn demo_search_matches_artist() {
        let result = search_demo(&SearchRequest {
            keyword: "soundhelix".into(),
            search_type: SearchType::Song,
            source: None,
            page: 1,
            page_size: 20,
        });

        assert!(!result.songs.is_empty());
        assert!(result.songs.iter().all(|song| song.play_url.is_some()));
    }

    #[test]
    fn demo_search_respects_source_filter() {
        let result = search_demo(&SearchRequest {
            keyword: "soundhelix".into(),
            search_type: SearchType::Song,
            source: Some("other".into()),
            page: 1,
            page_size: 20,
        });

        assert!(result.songs.is_empty());
    }
}
