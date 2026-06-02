import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Playlist, Song } from '@/types'
import * as api from '@/api/commands'

export const usePlaylistStore = defineStore('playlist', () => {
  const playlists = ref<Playlist[]>([])
  const selectedSongs = ref<Song[]>([])
  const selectedPlaylistId = ref<string | null>(null)
  const loading = ref(false)
  const songsLoading = ref(false)

  async function refresh() {
    loading.value = true
    try {
      playlists.value = await api.listPlaylists()
    } finally {
      loading.value = false
    }
  }

  async function create(name: string) {
    const pl = await api.createPlaylist(name)
    await refresh()
    return pl
  }

  async function remove(id: string) {
    await api.deletePlaylist(id)
    if (selectedPlaylistId.value === id) {
      selectedPlaylistId.value = null
      selectedSongs.value = []
    }
    await refresh()
  }

  async function rename(id: string, name: string) {
    await api.renamePlaylist(id, name)
    await refresh()
  }

  async function refreshSongs(playlistId: string) {
    selectedPlaylistId.value = playlistId
    songsLoading.value = true
    try {
      selectedSongs.value = await api.listPlaylistSongs(playlistId)
    } finally {
      songsLoading.value = false
    }
  }

  async function addSong(playlistId: string, song: Song) {
    await api.addSongToPlaylist(playlistId, song)
    if (selectedPlaylistId.value === playlistId) {
      await refreshSongs(playlistId)
    }
  }

  async function removeSong(playlistId: string, songId: string) {
    await api.removeSongFromPlaylist(playlistId, songId)
    if (selectedPlaylistId.value === playlistId) {
      await refreshSongs(playlistId)
    }
    await refresh()
  }

  return {
    playlists,
    selectedSongs,
    selectedPlaylistId,
    loading,
    songsLoading,
    refresh,
    create,
    remove,
    rename,
    refreshSongs,
    addSong,
    removeSong,
  }
})
