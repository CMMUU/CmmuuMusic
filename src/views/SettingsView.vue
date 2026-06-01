<script setup lang="ts">
import { onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/stores/settings'

const settings = useSettingsStore()
const { themeMode } = storeToRefs(settings)

onMounted(() => settings.load())

const themeOptions: { value: 'light' | 'dark' | 'system'; label: string }[] = [
  { value: 'dark', label: '深色' },
  { value: 'light', label: '浅色' },
  { value: 'system', label: '跟随系统' },
]
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

.seg__item--active {
  background: var(--accent);
  color: #fff;
}
</style>
