<script setup lang="ts">
import { onMounted, ref } from 'vue'
import * as api from '@/api/commands'
import { usePlayerStore } from '@/stores/player'
import type { PlayHistoryRecord } from '@/types'

const player = usePlayerStore()
const history = ref<PlayHistoryRecord[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const message = ref<string | null>(null)

onMounted(() => {
  void refresh()
})

async function refresh() {
  loading.value = true
  error.value = null
  message.value = null
  try {
    history.value = await api.listPlayHistory(100)
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}

async function play(record: PlayHistoryRecord) {
  error.value = null
  message.value = null
  try {
    await player.playRemoteSong(record.song)
    message.value = `正在播放：${record.song.title}`
  } catch (e) {
    error.value = String(e)
  }
}

function addToQueue(record: PlayHistoryRecord) {
  error.value = null
  player.addToQueue(record.song)
  message.value = `已加入队列：${record.song.title}`
}

function addAllToQueue() {
  error.value = null
  const songs = history.value.map((record) => record.song)
  player.addManyToQueue(songs)
  message.value = `已加入 ${songs.length} 首最近播放歌曲到队列`
}
</script>

<template>
  <div class="recent">
    <header class="recent__header">
      <h1>最近播放</h1>
      <div class="recent__actions">
        <button class="ghost-btn" :disabled="!history.length" @click="addAllToQueue">
          全部加入队列
        </button>
        <button class="ghost-btn" :disabled="loading" @click="refresh">
          {{ loading ? '刷新中…' : '刷新' }}
        </button>
      </div>
    </header>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="message" class="message">{{ message }}</p>

    <ul v-if="history.length" class="history-list">
      <li v-for="record in history" :key="record.id" class="history-item">
        <div class="history-item__main">
          <span class="history-item__title">{{ record.song.title }}</span>
          <span class="history-item__meta">
            {{ record.song.artist ?? '未知艺人' }} · {{ record.song.album ?? '未知专辑' }} · {{ record.playedAt }}
          </span>
        </div>
        <div class="history-item__actions">
          <button class="text-btn" @click="addToQueue(record)">加入队列</button>
          <button class="text-btn" :disabled="!record.song.playUrl" @click="play(record)">播放</button>
        </div>
      </li>
    </ul>

    <div v-else-if="!loading" class="empty">
      <p>还没有播放历史</p>
      <p class="empty__hint">播放搜索结果或本地音乐后会记录在这里。</p>
    </div>
  </div>
</template>

<style scoped>
.recent {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.recent__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.recent__actions,
.history-item__actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.recent h1 {
  font-size: 22px;
  font-weight: 700;
}

.ghost-btn {
  padding: 8px 16px;
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 600;
}

.ghost-btn:disabled,
.text-btn:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.error,
.message {
  font-size: 13px;
  padding: 10px 14px;
  border-radius: var(--radius-md);
}

.error {
  color: var(--error);
  background: rgba(255, 69, 58, 0.1);
}

.message {
  color: var(--success);
  background: rgba(52, 199, 89, 0.1);
}

.history-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.history-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 14px;
  border-radius: var(--radius-md);
}

.history-item:hover {
  background: var(--bg-secondary);
}

.history-item__main {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.history-item__title {
  font-size: 14px;
  font-weight: 600;
}

.history-item__meta {
  color: var(--text-secondary);
  font-size: 12px;
}

.text-btn {
  flex-shrink: 0;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 12px;
}

.text-btn:hover:not(:disabled) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.empty {
  margin-top: 60px;
  text-align: center;
  color: var(--text-secondary);
}

.empty__hint {
  margin-top: 8px;
  color: var(--text-tertiary);
  font-size: 13px;
}
</style>
