<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/stores/settings'
import * as api from '@/api/commands'
import type { PluginRecord } from '@/types'

const settings = useSettingsStore()
const { themeMode, useCustomTitleBar } = storeToRefs(settings)
const plugins = ref<PluginRecord[]>([])
const pluginsError = ref<string | null>(null)

onMounted(async () => {
  await settings.load()
  await refreshPlugins()
})

const themeOptions: { value: 'light' | 'dark' | 'system'; label: string }[] = [
  { value: 'dark', label: '深色' },
  { value: 'light', label: '浅色' },
  { value: 'system', label: '跟随系统' },
]

async function refreshPlugins() {
  pluginsError.value = null
  try {
    plugins.value = await api.listPlugins()
  } catch (e) {
    pluginsError.value = String(e)
  }
}

async function togglePlugin(plugin: PluginRecord) {
  pluginsError.value = null
  try {
    const changed = await api.setPluginEnabled(plugin.info.id, !plugin.enabled)
    if (!changed) {
      pluginsError.value = '内置音源当前仅登记元数据，不能直接启用执行。'
    }
    await refreshPlugins()
  } catch (e) {
    pluginsError.value = String(e)
  }
}
</script>

<template>
  <div class="settings">
    <h1>设置</h1>

    <section class="setting-group">
      <h2 class="setting-group__title">外观</h2>
      <div class="setting-row">
        <span class="setting-row__label">主题</span>
        <div class="seg">
          <button
            v-for="opt in themeOptions"
            :key="opt.value"
            class="seg__item"
            :class="{ 'seg__item--active': themeMode === opt.value }"
            @click="settings.setThemeMode(opt.value)"
          >
            {{ opt.label }}
          </button>
        </div>
      </div>
      <div class="setting-row">
        <span class="setting-row__label">自定义标题栏</span>
        <button
          class="toggle"
          :class="{ 'toggle--active': useCustomTitleBar }"
          @click="settings.setUseCustomTitleBar(!useCustomTitleBar)"
        >
          {{ useCustomTitleBar ? '已开启' : '系统默认' }}
        </button>
      </div>
    </section>

    <section class="setting-group">
      <h2 class="setting-group__title">内置音源</h2>
      <p v-if="pluginsError" class="setting-error">{{ pluginsError }}</p>
      <div v-else-if="plugins.length" class="plugin-list">
        <article v-for="plugin in plugins" :key="plugin.info.id" class="plugin-card">
          <div class="plugin-card__main">
            <div class="plugin-card__title">
              <span>{{ plugin.info.name }}</span>
              <span class="plugin-card__version">v{{ plugin.info.version }}</span>
            </div>
            <p>{{ plugin.info.description }}</p>
            <p class="plugin-card__meta">
              {{ plugin.info.pluginType.toUpperCase() }} · {{ plugin.info.author }} · {{ plugin.filePath }}
            </p>
            <div class="source-tags">
              <span v-for="source in plugin.sources" :key="source.id" class="source-tag">
                {{ source.name }}
              </span>
            </div>
          </div>
          <div class="plugin-card__side">
            <span class="plugin-card__status">{{ plugin.enabled ? '已启用' : '仅内置' }}</span>
            <button class="plugin-card__action" @click="togglePlugin(plugin)">
              {{ plugin.enabled ? '禁用' : '启用' }}
            </button>
          </div>
        </article>
      </div>
      <p v-else class="setting-row__value">暂无内置音源</p>
    </section>

    <section class="setting-group">
      <h2 class="setting-group__title">关于</h2>
      <div class="setting-row">
        <span class="setting-row__label">版本</span>
        <span class="setting-row__value">0.1.0 (Phase 0)</span>
      </div>
    </section>
  </div>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings h1 {
  font-size: 22px;
  font-weight: 700;
}

.setting-group {
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  padding: 18px 20px;
}

.setting-group__title {
  font-size: 13px;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 14px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.setting-row__label {
  font-size: 14px;
}

.setting-row__value {
  font-size: 13px;
  color: var(--text-secondary);
}

.seg {
  display: flex;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  padding: 3px;
  gap: 2px;
}

.seg__item {
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  font-size: 13px;
  color: var(--text-secondary);
  transition: all 0.15s ease;
}

.seg__item--active,
.toggle--active {
  background: var(--accent);
  color: #fff;
}

.toggle {
  padding: 6px 14px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-size: 13px;
  transition: all 0.15s ease;
}

.toggle:hover {
  color: var(--text-primary);
}

.setting-error {
  color: var(--error);
  font-size: 13px;
}

.plugin-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.plugin-card {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  padding: 14px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
}

.plugin-card__main {
  min-width: 0;
}

.plugin-card__title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
}

.plugin-card__version,
.plugin-card__meta {
  color: var(--text-tertiary);
  font-size: 12px;
}

.plugin-card p {
  margin-top: 6px;
  color: var(--text-secondary);
  font-size: 13px;
}

.plugin-card__side {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.plugin-card__status {
  flex-shrink: 0;
  align-self: flex-start;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  background: rgba(124, 108, 240, 0.16);
  color: var(--accent);
  font-size: 12px;
}

.plugin-card__action {
  padding: 5px 10px;
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: 12px;
}

.plugin-card__action:hover {
  color: var(--text-primary);
}

.source-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 10px;
}

.source-tag {
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  background: rgba(255, 255, 255, 0.06);
  color: var(--text-secondary);
  font-size: 12px;
}
</style>
