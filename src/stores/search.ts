import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { SearchResult, SearchType } from '@/types'
import * as api from '@/api/commands'

export const useSearchStore = defineStore('search', () => {
  const keyword = ref('')
  const searchType = ref<SearchType>('song')
  const source = ref<string>('all')
  const result = ref<SearchResult | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function search() {
    if (!keyword.value.trim() && searchType.value !== 'playlist') return
    loading.value = true
    error.value = null
    try {
      result.value = await api.searchMusic({
        keyword: keyword.value,
        searchType: searchType.value,
        source: source.value === 'all' ? null : source.value,
        page: 1,
        pageSize: 20,
      })
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function loadMore() {
    if ((!keyword.value.trim() && searchType.value !== 'playlist') || !result.value?.hasMore) return
    loading.value = true
    error.value = null
    try {
      const next = await api.searchMusic({
        keyword: keyword.value,
        searchType: searchType.value,
        source: source.value === 'all' ? null : source.value,
        page: result.value.page + 1,
        pageSize: 20,
      })
      result.value = {
        ...next,
        songs: [...result.value.songs, ...next.songs],
        playlists: [...result.value.playlists, ...next.playlists],
      }
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  return { keyword, searchType, source, result, loading, error, search, loadMore }
})
