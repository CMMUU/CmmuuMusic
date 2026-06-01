import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { PlaybackState, PlayMode, Song } from '@/types'
import * as api from '@/api/commands'

export const usePlayerStore = defineStore('player', () => {
  const currentSong = ref<Song | null>(null)
  const state = ref<PlaybackState>('idle')
  const position = ref(0)
  const duration = ref<number | null>(null)
  const volume = ref(1.0)
  const playMode = ref<PlayMode>('sequential')

  let pollTimer: number | null = null

  const isPlaying = computed(() => state.value === 'playing')
  const progress = computed(() =>
    duration.value && duration.value > 0
      ? position.value / duration.value
      : 0,
  )

  /** 从后端同步一次播放状态 */
  async function syncStatus() {
    try {
      const s = await api.getPlaybackStatus()
      state.value = s.state
      position.value = s.position
      duration.value = s.duration ?? null
      volume.value = s.volume
    } catch (e) {
      console.error('同步播放状态失败', e)
    }
  }

  function startPolling() {
    if (pollTimer !== null) return
    pollTimer = window.setInterval(syncStatus, 500)
  }

  function stopPolling() {
    if (pollTimer !== null) {
      window.clearInterval(pollTimer)
      pollTimer = null
    }
  }

  /** 播放本地文件 */
  async function playLocalFile(path: string, song?: Song) {
    await api.playFile(path)
    if (song) currentSong.value = song
    startPolling()
    await syncStatus()
  }

  async function playRemoteSong(song: Song) {
    if (!song.playUrl) throw new Error('该歌曲没有可播放 URL')
    await api.playUrl(song.playUrl)
    currentSong.value = song
    startPolling()
    await syncStatus()
  }

  async function togglePause() {
    await api.togglePause()
    await syncStatus()
  }

  async function stop() {
    await api.stop()
    stopPolling()
    await syncStatus()
  }

  async function seek(positionSecs: number) {
    await api.seek(positionSecs)
    position.value = positionSecs
  }

  async function setVolume(v: number) {
    volume.value = v
    await api.setVolume(v)
  }

  function toggleShuffle() {
    playMode.value =
      playMode.value === 'shuffle' ? 'sequential' : 'shuffle'
  }

  function cyclePlayMode() {
    const order: PlayMode[] = ['sequential', 'loop', 'shuffle', 'single']
    const idx = order.indexOf(playMode.value)
    playMode.value = order[(idx + 1) % order.length]
  }

  return {
    currentSong,
    state,
    position,
    duration,
    volume,
    playMode,
    isPlaying,
    progress,
    syncStatus,
    startPolling,
    stopPolling,
    playLocalFile,
    playRemoteSong,
    togglePause,
    stop,
    seek,
    setVolume,
    toggleShuffle,
    cyclePlayMode,
  }
})
