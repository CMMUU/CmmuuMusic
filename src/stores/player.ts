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
  const queue = ref<Song[]>([])
  const currentIndex = ref(-1)
  const lyrics = ref<string | null>(null)

  let pollTimer: number | null = null

  const isPlaying = computed(() => state.value === 'playing')
  const hasPrevious = computed(() => queue.value.length > 0 && currentIndex.value > 0)
  const hasNext = computed(() => queue.value.length > 0 && currentIndex.value < queue.value.length - 1)
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

  async function loadLyrics(songId: string) {
    lyrics.value = await api.getLyrics(songId)
  }

  async function recordHistory(song: Song) {
    try {
      await api.recordPlayHistory(song, position.value || null)
    } catch (e) {
      console.error('记录播放历史失败', e)
    }
  }

  function setCurrentFromQueue(song: Song) {
    const idx = queue.value.findIndex((item) => item.id === song.id)
    if (idx >= 0) {
      currentIndex.value = idx
    }
  }

  function addToQueue(song: Song) {
    if (!queue.value.some((item) => item.id === song.id)) {
      queue.value.push(song)
    }
  }

  function addManyToQueue(songs: Song[]) {
    for (const song of songs) addToQueue(song)
  }

  async function playQueueSong(index: number) {
    const song = queue.value[index]
    if (!song) return
    currentIndex.value = index
    if (song.playUrl) {
      await playRemoteSong(song)
    } else {
      throw new Error('队列中的歌曲暂缺可播放 URL')
    }
  }

  /** 播放本地文件 */
  async function playLocalFile(path: string, song?: Song) {
    await api.playFile(path)
    if (song) {
      currentSong.value = song
      addToQueue(song)
      setCurrentFromQueue(song)
      await recordHistory(song)
      await loadLyrics(song.id)
    }
    startPolling()
    await syncStatus()
  }

  async function playRemoteSong(song: Song) {
    if (!song.playUrl) throw new Error('该歌曲没有可播放 URL')
    await api.playUrl(song.playUrl)
    currentSong.value = song
    addToQueue(song)
    setCurrentFromQueue(song)
    await recordHistory(song)
    await loadLyrics(song.id)
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

  async function playPrevious() {
    if (!hasPrevious.value) return
    await playQueueSong(currentIndex.value - 1)
  }

  async function playNext() {
    if (playMode.value === 'single' && currentIndex.value >= 0) {
      await playQueueSong(currentIndex.value)
      return
    }

    if (playMode.value === 'shuffle' && queue.value.length > 1) {
      let next = currentIndex.value
      while (next === currentIndex.value) {
        next = Math.floor(Math.random() * queue.value.length)
      }
      await playQueueSong(next)
      return
    }

    if (hasNext.value) {
      await playQueueSong(currentIndex.value + 1)
    } else if (playMode.value === 'loop' && queue.value.length > 0) {
      await playQueueSong(0)
    }
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
    queue,
    currentIndex,
    lyrics,
    hasPrevious,
    hasNext,
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
    addToQueue,
    addManyToQueue,
    playPrevious,
    playNext,
    toggleShuffle,
    cyclePlayMode,
  }
})
