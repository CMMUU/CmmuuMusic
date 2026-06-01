// 与 Rust 端 src-tauri/src/types 对齐的前端类型定义。

export type Quality =
  | '128k'
  | '320k'
  | 'flac'
  | 'flac24bit'
  | 'hires'
  | 'atmos'
  | 'master'

export interface Song {
  id: string
  source: string
  title: string
  artist?: string | null
  album?: string | null
  coverUrl?: string | null
  duration?: number | null
  lyricText?: string | null
  playUrl?: string | null
}

export interface Playlist {
  id: string
  name: string
  description?: string | null
  coverUrl?: string | null
  songs: Song[]
  createdAt: string
  updatedAt: string
}

export type PlayMode = 'sequential' | 'loop' | 'shuffle' | 'single'

export type PlaybackState =
  | 'idle'
  | 'loading'
  | 'playing'
  | 'paused'
  | 'ended'
  | 'error'

export interface PlaybackStatus {
  state: PlaybackState
  position: number
  duration?: number | null
  volume: number
}

export type SearchType = 'song' | 'album' | 'artist' | 'playlist'

export interface SearchRequest {
  keyword: string
  searchType?: SearchType
  source?: string | null
  page?: number
  pageSize?: number
}

export interface SearchResult {
  songs: Song[]
  total: number
  page: number
  hasMore: boolean
}

export type PluginType = 'cmmuu' | 'lx'
export type PluginStatus = 'ready' | 'disabled' | 'error'

export interface PluginInfo {
  id: string
  name: string
  description: string
  version: string
  author: string
  homepage: string
  pluginType: PluginType
}

export interface MusicSource {
  id: string
  name: string
  qualities: string[]
}

export interface PluginRecord {
  info: PluginInfo
  sources: MusicSource[]
  filePath: string
  enabled: boolean
  status: PluginStatus
  installedAt: string
  updatedAt: string
}
