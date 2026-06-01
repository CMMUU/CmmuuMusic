import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as api from '@/api/commands'

type ThemeMode = 'light' | 'dark' | 'system'

export const useSettingsStore = defineStore('settings', () => {
  const themeMode = ref<ThemeMode>('dark')
  const volume = ref(1.0)
  const loaded = ref(false)

  async function load() {
    try {
      const theme = await api.getSetting('themeMode')
      if (theme) themeMode.value = theme as ThemeMode
      const vol = await api.getSetting('volume')
      if (vol) volume.value = Number(vol)
    } catch (e) {
      console.error('加载设置失败', e)
    } finally {
      loaded.value = true
    }
  }

  async function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode
    await api.setSetting('themeMode', mode)
  }

  async function setVolume(v: number) {
    volume.value = v
    await api.setSetting('volume', String(v))
  }

  return { themeMode, volume, loaded, load, setThemeMode, setVolume }
})
