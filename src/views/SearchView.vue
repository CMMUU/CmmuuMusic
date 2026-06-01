<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useSearchStore } from '@/stores/search'
import { usePlayerStore } from '@/stores/player'
import { usePlaylistStore } from '@/stores/playlist'
import type { Song } from '@/types'

const search = useSearchStore()
const player = usePlayerStore()
const playlist = usePlaylistStore()

const { keyword, result, loading, error } = storeToRefs(search)
const { playlists } = storeToRefs(playlist)
const selectedPlaylistId = ref<string>('')
const actionMessage = ref<string | null>(null)
const actionError = ref<string | null>(null)

const canAddToPlaylist = computed(() => playlists.value.length > 0)

onMounted(async () => {
  await playlist.refresh()
  selectedPlaylistId.value = playlists.value[0]?.id ?? ''
})

async function play(song: Song) {
  actionError.value = null
  actionMessage.value = null
  try {
    await player.playRemoteSong(song)
    actionMessage.value = `正在播放：${song.title}`
  } catch (e) {
    actionError.value = String(e)
  }
}

async function addToPlaylist(song: Song) {
  actionError.value = null
  actionMessage.value = null
  if (!selectedPlaylistId.value) {
    actionError.value = '请先创建并选择一个歌单'
    return
  }

  try {
    await playlist.addSong(selectedPlaylistId.value, song)
    actionMessage.value = `已加入歌单：${song.title}`
  } catch (e) {
    actionError.value = String(e)
  }
}

function addToQueue(song: Song) {
  actionError.value = null
  player.addToQueue(song)
  actionMessage.value = `已加入播放队列：${song.title}`
}
</script>

<template>
  <div class="search">
    <div class="search__bar">
      <input
        v-model="keyword"
        class="search__input"
        type="text"
        placeholder="搜索 demo、SoundHelix…"
        @keyup.enter="search.search()"
      />
      <button class="search__btn" :disabled="loading" @click="search.search()">
        {{ loading ? '搜索中…' : '搜索' }}
      </button>
    </div>

    <div class="search__tools">
      <span class="search__source">当前音源：内置 demo</span>
      <select v-model="selectedPlaylistId" class="playlist-select" :disabled="!canAddToPlaylist">
        <option value="">{{ canAddToPlaylist ? '选择歌单' : '请先创建歌单' }}</option>
        <option v-for="item in playlists" :key="item.id" :value="item.id">
          {{ item.name }}
        </option>
      </select>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="actionError" class="error">{{ actionError }}</p>
    <p v-if="actionMessage" class="message">{{ actionMessage }}</p>

    <div v-if="result && result.songs.length" class="results">
      <div v-for="song in result.songs" :key="song.id" class="result-item">
        <div class="result-item__main">
          <div class="result-item__title">{{ song.title }}</div>
          <div class="result-item__meta">
            {{ song.artist ?? '未知艺人' }} · {{ song.album ?? '未知专辑' }} · {{ song.source }}
          </div>
        </div>
        <div class="result-item__actions">
          <button class="text-btn" :disabled="!song.playUrl" @click="play(song)">播放</button>
          <button class="text-btn" @click="addToQueue(song)">加入队列</button>
          <button class="text-btn" :disabled="!selectedPlaylistId" @click="addToPlaylist(song)">
            加入歌单
          </button>
        </div>
      </div>
    </div>

    <div v-else-if="result" class="empty">暂无匹配的 demo 歌曲</div>

    <div v-else class="empty">输入关键词开始搜索，试试 SoundHelix 或 demo</div>
  </div>
</template>

<style scoped>
.search {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.search__bar {
  display: flex;
  gap: 10px;
}

.search__input {
  flex: 1;
  padding: 12px 16px;
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  border: 1px solid transparent;
  transition: border 0.15s ease;
}

.search__input:focus,
.playlist-select:focus {
  border-color: var(--accent);
  outline: none;
}

.search__btn {
  padding: 0 24px;
  border-radius: var(--radius-md);
  background: linear-gradient(135deg, var(--accent-start), var(--accent-end));
  color: #fff;
  font-weight: 600;
  font-size: 13px;
}

.search__btn:disabled,
.text-btn:disabled,
.playlist-select:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.search__tools {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.search__source {
  color: var(--text-tertiary);
  font-size: 13px;
}

.playlist-select {
  min-width: 180px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  border: 1px solid transparent;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.error {
  color: var(--error);
  font-size: 13px;
}

.message {
  color: var(--success);
  font-size: 13px;
}

.results {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.result-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 14px;
  border-radius: var(--radius-md);
  font-size: 14px;
}

.result-item:hover {
  background: var(--bg-secondary);
}

.result-item__main {
  min-width: 0;
}

.result-item__title {
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-item__meta {
  margin-top: 4px;
  color: var(--text-secondary);
  font-size: 12px;
}

.result-item__actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.text-btn {
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
  color: var(--text-tertiary);
  font-size: 14px;
}
</style>
