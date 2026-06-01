<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()

const minimize = () => appWindow.minimize()
const toggleMaximize = () => appWindow.toggleMaximize()
const close = () => appWindow.close()
</script>

<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title-bar__brand" data-tauri-drag-region>
      <span class="title-bar__logo">Cmmuu</span>
      <span class="title-bar__sub">Music</span>
    </div>
    <div class="title-bar__controls">
      <button class="ctrl" title="最小化" @click="minimize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="2" y1="6" x2="10" y2="6" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
      <button class="ctrl" title="最大化" @click="toggleMaximize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="2.5" y="2.5" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
      <button class="ctrl ctrl--close" title="关闭" @click="close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="3" y1="3" x2="9" y2="9" stroke="currentColor" stroke-width="1.2" />
          <line x1="9" y1="3" x2="3" y2="9" stroke="currentColor" stroke-width="1.2" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  flex-shrink: 0;
}

.title-bar__brand {
  display: flex;
  align-items: baseline;
  gap: 8px;
  padding-left: 8px;
}

.title-bar__logo {
  font-weight: 700;
  font-size: 16px;
  background: linear-gradient(90deg, var(--accent-start), var(--accent-end));
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.title-bar__sub {
  font-size: 11px;
  color: var(--text-tertiary);
  letter-spacing: 0.5px;
}

.title-bar__controls {
  display: flex;
  gap: 2px;
}

.ctrl {
  width: 32px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  transition: background 0.15s ease;
}

.ctrl:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.ctrl--close:hover {
  background: var(--error);
  color: #fff;
}
</style>
