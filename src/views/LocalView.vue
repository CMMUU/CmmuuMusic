<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { usePlayerStore } from '@/stores/player'
import type { Song } from '@/types'

const player = usePlayerStore()
const lastError = ref<string | null>(null)
const localFiles = ref<{ path: string; name: string }[]>([])

async function pickFiles() {
  lastError.value = null
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: '音频文件',
        extensions: ['mp3', 'flac', 'm4a', 'aac', 'wav', 'ogg', 'opus', 'wma'],
      },
    ],
  })
  if (!selected) return
  const paths = Array.isArray(selected) ? selected : [selected]
  for (const p of paths) {
    const name = p.split(/[\\/]/).pop() ?? p
    if (!localFiles.value.some((f) => f.path === p)) {
      localFiles.value.push({ path: p, name })
    }
  }
}

async function play(file: { path: string; name: string }) {
  lastError.value = null
  try {
    const song: Song = {
      id: `local:${file.path}`,
      source: 'local',
      title: file.name,
      artist: '本地文件',
    }
    await player.playLocalFile(file.path, song)
  } catch (e) {
    lastError.value = String(e)
  }
}
</script>

<template>
  <div class="local">
    <header class="local__header">
      <h1>本地音乐</h1>
      <button class="btn-primary" @click="pickFiles">添加文件</button>
    </header>

    <p v-if="lastError" class="error">{{ lastError }}</p>

    <ul v-if="localFiles.length" class="file-list">
      <li
        v-for="file in localFiles"
        :key="file.path"
        class="file-item"
        @dblclick="play(file)"
      >
        <span class="file-item__icon">♪</span>
        <span class="file-item__name">{{ file.name }}</span>
        <button class="file-item__play" @click="play(file)">播放</button>
      </li>
    </ul>

    <div v-else class="empty">
      <p>还没有本地文件</p>
      <p class="empty__hint">点击「添加文件」选择音频，双击列表项即可播放</p>
    </div>
  </div>
</template>

<style scoped>
.local {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.local__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.local__header h1 {
  font-size: 22px;
  font-weight: 700;
}

.btn-primary {
  padding: 8px 16px;
  border-radius: var(--radius-md);
  background: linear-gradient(135deg, var(--accent-start), var(--accent-end));
  color: #fff;
  font-size: 13px;
  font-weight: 600;
  transition: filter 0.15s ease;
}

.btn-primary:hover {
  filter: brightness(1.1);
}

.error {
  color: var(--error);
  font-size: 13px;
  background: rgba(255, 69, 58, 0.1);
  padding: 10px 14px;
  border-radius: var(--radius-md);
}

.file-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 12px 14px;
  border-radius: var(--radius-md);
  cursor: default;
  transition: background 0.15s ease;
}

.file-item:hover {
  background: var(--bg-secondary);
}

.file-item__icon {
  color: var(--accent);
}

.file-item__name {
  flex: 1;
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-item__play {
  opacity: 0;
  padding: 4px 12px;
  border-radius: var(--radius-sm);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-size: 12px;
  transition: all 0.15s ease;
}

.file-item:hover .file-item__play {
  opacity: 1;
}

.file-item__play:hover {
  color: var(--text-primary);
}

.empty {
  margin-top: 60px;
  text-align: center;
  color: var(--text-secondary);
}

.empty__hint {
  margin-top: 8px;
  font-size: 13px;
  color: var(--text-tertiary);
}
</style>
