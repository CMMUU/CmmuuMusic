import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { PluginRecord } from '@/types'
import * as api from '@/api/commands'

export const usePluginStore = defineStore('plugin', () => {
  const plugins = ref<PluginRecord[]>([])
  const loading = ref(false)

  async function refresh() {
    loading.value = true
    try {
      plugins.value = await api.listPlugins()
    } finally {
      loading.value = false
    }
  }

  return { plugins, loading, refresh }
})
