import { invoke } from '@tauri-apps/api/core'
import type {
  Playlist,
  PlayHistoryRecord,
  PlaybackStatus,
  PluginRecord,
  PluginType,
  SearchRequest,
  SearchResult,
  Song,
} from '@/types'

// 连通性测试
export const greet = (name: string) => invoke<string>('greet', { name })

// 播放控制
export const playFile = (path: string) => invoke<void>('play_file', { path })
export const playUrl = (url: string) => invoke<void>('play_url', { url })
export const togglePause = () => invoke<void>('toggle_pause')
export const stop = () => invoke<void>('stop')
export const seek = (positionSecs: number) =>
  invoke<void>('seek', { positionSecs })
export const setVolume = (volume: number) =>
  invoke<void>('set_volume', { volume })
export const getPlaybackStatus = () =>
  invoke<PlaybackStatus>('get_playback_status')

// 播放列表
export const createPlaylist = (name: string) =>
  invoke<Playlist>('create_playlist', { name })
export const listPlaylists = () => invoke<Playlist[]>('list_playlists')
export const deletePlaylist = (id: string) =>
  invoke<boolean>('delete_playlist', { id })
export const renamePlaylist = (id: string, name: string) =>
  invoke<boolean>('rename_playlist', { id, name })
export const addSongToPlaylist = (playlistId: string, song: Song) =>
  invoke<void>('add_song_to_playlist', { playlistId, song })
export const listPlaylistSongs = (playlistId: string) =>
  invoke<Song[]>('list_playlist_songs', { playlistId })
export const removeSongFromPlaylist = (playlistId: string, songId: string) =>
  invoke<boolean>('remove_song_from_playlist', { playlistId, songId })

// 播放历史
export const recordPlayHistory = (song: Song, durationPlayed?: number | null) =>
  invoke<PlayHistoryRecord>('record_play_history', { song, durationPlayed })
export const listPlayHistory = (limit?: number) =>
  invoke<PlayHistoryRecord[]>('list_play_history', { limit })
export const listRecentSongs = (limit?: number) =>
  invoke<Song[]>('list_recent_songs', { limit })

// 设置
export const getSetting = (key: string) =>
  invoke<string | null>('get_setting', { key })
export const setSetting = (key: string, value: string) =>
  invoke<void>('set_setting', { key, value })

// 插件
export interface RegisterLocalPluginRequest {
  id: string
  name: string
  version: string
  author?: string | null
  pluginType: PluginType
  filePath: string
}

export const listPlugins = () => invoke<PluginRecord[]>('list_plugins')
export const registerLocalPlugin = (request: RegisterLocalPluginRequest) =>
  invoke<void>('register_local_plugin', { request })
export const setPluginEnabled = (pluginId: string, enabled: boolean) =>
  invoke<boolean>('set_plugin_enabled', { pluginId, enabled })

// 搜索
export const searchMusic = (request: SearchRequest) =>
  invoke<SearchResult>('search_music', { request })
export const listSourcePlaylistSongs = (source: string, playlistId: string) =>
  invoke<Song[]>('list_source_playlist_songs', { source, playlistId })

// 歌词
export const getLyrics = (songId: string) =>
  invoke<string | null>('get_lyrics', { songId })
export const setLyrics = (songId: string, lyricText: string) =>
  invoke<boolean>('set_lyrics', { songId, lyricText })
