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
  let lastSyncedState: PlaybackState = 'idle'
  let handlingEnded = false

  const isPlaying = computed(() => state.value === 'playing')
  const hasPrevious = computed(() => {
    if (queue.value.length === 0 || currentIndex.value < 0) return false
    if (playMode.value === 'single') return true
    if (playMode.value === 'shuffle') return queue.value.length > 1
    if (playMode.value === 'loop') return queue.value.length > 1
    return currentIndex.value > 0
  })
  const hasNext = computed(() => {
    if (queue.value.length === 0 || currentIndex.value < 0) return false
    if (playMode.value === 'single') return true
    if (playMode.value === 'shuffle') return queue.value.length > 1
    if (playMode.value === 'loop') return queue.value.length > 1
    return currentIndex.value < queue.value.length - 1
  })
  const progress = computed(() =>
    duration.value && duration.value > 0
      ? position.value / duration.value
      : 0,
  )

  /** 从后端同步一次播放状态 */
  async function syncStatus() {
    try {
      const s = await api.getPlaybackStatus()
      const previousState = lastSyncedState
      state.value = s.state
      position.value = s.position
      duration.value = s.duration ?? null
      volume.value = s.volume
      lastSyncedState = s.state
      if (s.state === 'ended' && previousState !== 'ended') {
        await handlePlaybackEnded()
      }
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

  async function loadLyrics(song: Song) {
    lyrics.value = song.lyricText ?? await api.getLyrics(song.id)
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

  function removeFromQueue(index: number) {
    if (index < 0 || index >= queue.value.length) return
    queue.value.splice(index, 1)
    if (queue.value.length === 0) {
      currentIndex.value = -1
      return
    }
    if (index < currentIndex.value) {
      currentIndex.value -= 1
    } else if (index === currentIndex.value) {
      currentIndex.value = Math.min(index, queue.value.length - 1)
    }
  }

  function clearQueue() {
    queue.value = []
    currentIndex.value = -1
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
      await loadLyrics(song)
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
    await loadLyrics(song)
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
    const previousIndex = getPreviousIndex()
    if (previousIndex === null) return
    await playQueueSong(previousIndex)
  }

  async function playNext() {
    const nextIndex = getNextIndex()
    if (nextIndex === null) return
    await playQueueSong(nextIndex)
  }

  function getPreviousIndex(): number | null {
    if (queue.value.length === 0 || currentIndex.value < 0) return null
    if (playMode.value === 'single') return currentIndex.value
    if (playMode.value === 'shuffle') return pickRandomIndex()
    if (currentIndex.value > 0) return currentIndex.value - 1
    if (playMode.value === 'loop' && queue.value.length > 1) return queue.value.length - 1
    return null
  }

  function getNextIndex(): number | null {
    if (queue.value.length === 0 || currentIndex.value < 0) return null
    if (playMode.value === 'single') return currentIndex.value
    if (playMode.value === 'shuffle') return pickRandomIndex()
    if (currentIndex.value < queue.value.length - 1) return currentIndex.value + 1
    if (playMode.value === 'loop' && queue.value.length > 1) return 0
    return null
  }

  function pickRandomIndex(): number | null {
    if (queue.value.length <= 1) return null
    let next = currentIndex.value
    while (next === currentIndex.value) {
      next = Math.floor(Math.random() * queue.value.length)
    }
    return next
  }

  async function handlePlaybackEnded() {
    if (handlingEnded) return
    handlingEnded = true
    try {
      const nextIndex = getNextIndex()
      if (nextIndex === null) {
        stopPolling()
        return
      }
      await playQueueSong(nextIndex)
    } catch (e) {
      console.error('自动播放下一首失败', e)
    } finally {
      handlingEnded = false
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
    removeFromQueue,
    clearQueue,
    playQueueSong,
    playPrevious,
    playNext,
    toggleShuffle,
    cyclePlayMode,
  }
})
