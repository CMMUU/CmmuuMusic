import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as api from '@/api/commands'

type ThemeMode = 'light' | 'dark' | 'system'

export const useSettingsStore = defineStore('settings', () => {
  const themeMode = ref<ThemeMode>('dark')
  const volume = ref(1.0)
  const useCustomTitleBar = ref(false)
  const loaded = ref(false)

  function applyTheme(mode: ThemeMode) {
    document.documentElement.dataset.theme = mode
  }

  async function load() {
    try {
      const theme = await api.getSetting('themeMode')
      if (theme) themeMode.value = theme as ThemeMode
      const vol = await api.getSetting('volume')
      if (vol) volume.value = Number(vol)
      const customTitleBar = await api.getSetting('useCustomTitleBar')
      if (customTitleBar) useCustomTitleBar.value = customTitleBar === 'true'
      applyTheme(themeMode.value)
    } catch (e) {
      console.error('加载设置失败', e)
    } finally {
      loaded.value = true
    }
  }

  async function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode
    applyTheme(mode)
    await api.setSetting('themeMode', mode)
  }

  async function setVolume(v: number) {
    volume.value = v
    await api.setSetting('volume', String(v))
  }

  async function setUseCustomTitleBar(enabled: boolean) {
    useCustomTitleBar.value = enabled
    await api.setSetting('useCustomTitleBar', String(enabled))
  }

  return {
    themeMode,
    volume,
    useCustomTitleBar,
    loaded,
    load,
    setThemeMode,
    setVolume,
    setUseCustomTitleBar,
    applyTheme,
  }
})
