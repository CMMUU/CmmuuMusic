import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Playlist } from '@/types'
import * as api from '@/api/commands'

export const usePlaylistStore = defineStore('playlist', () => {
  const playlists = ref<Playlist[]>([])
  const loading = ref(false)

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
    await refresh()
  }

  return { playlists, loading, refresh, create, remove }
})
