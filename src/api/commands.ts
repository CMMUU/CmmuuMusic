import { invoke } from '@tauri-apps/api/core'
import type {
  Playlist,
  PlaybackStatus,
  PluginRecord,
  SearchRequest,
  SearchResult,
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

// 设置
export const getSetting = (key: string) =>
  invoke<string | null>('get_setting', { key })
export const setSetting = (key: string, value: string) =>
  invoke<void>('set_setting', { key, value })

// 插件
export const listPlugins = () => invoke<PluginRecord[]>('list_plugins')

// 搜索
export const searchMusic = (request: SearchRequest) =>
  invoke<SearchResult>('search_music', { request })

// 歌词
export const getLyrics = (songId: string) =>
  invoke<string | null>('get_lyrics', { songId })
