<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useSearchStore } from '@/stores/search'
import { usePlayerStore } from '@/stores/player'
import { usePlaylistStore } from '@/stores/playlist'
import * as api from '@/api/commands'
import type { PluginRecord, Song, SourcePlaylist } from '@/types'

const search = useSearchStore()
const player = usePlayerStore()
const playlist = usePlaylistStore()

const { keyword, searchType, source, result, loading, error } = storeToRefs(search)
const { playlists } = storeToRefs(playlist)
const selectedPlaylistId = ref<string>('')
const plugins = ref<PluginRecord[]>([])
const selectedSourcePlaylist = ref<SourcePlaylist | null>(null)
const sourcePlaylistSongs = ref<Song[]>([])
const sourcePlaylistLoading = ref(false)
const actionMessage = ref<string | null>(null)
const actionError = ref<string | null>(null)

const canAddToPlaylist = computed(() => playlists.value.length > 0)
const sourceOptions = computed(() => [
  { value: 'all', label: '全部音源' },
  { value: 'demo', label: '内置 demo' },
  ...plugins.value
    .filter((plugin) => plugin.enabled)
    .map((plugin) => ({ value: plugin.info.id, label: plugin.info.name })),
])

onMounted(async () => {
  await Promise.all([playlist.refresh(), refreshPlugins()])
  selectedPlaylistId.value = playlists.value[0]?.id ?? ''
  normalizeSource()
})

async function refreshPlugins() {
  plugins.value = await api.listPlugins()
}

function normalizeSource() {
  if (!sourceOptions.value.some((item) => item.value === source.value)) {
    source.value = 'all'
  }
}

async function searchCurrent() {
  normalizeSource()
  selectedSourcePlaylist.value = null
  sourcePlaylistSongs.value = []
  await search.search()
}

async function selectSourcePlaylist(item: SourcePlaylist) {
  actionError.value = null
  actionMessage.value = null
  selectedSourcePlaylist.value = item
  sourcePlaylistSongs.value = []
  sourcePlaylistLoading.value = true
  try {
    sourcePlaylistSongs.value = await api.listSourcePlaylistSongs(item.source, item.id)
    actionMessage.value = `已加载音源歌单：${item.name}`
  } catch (e) {
    actionError.value = String(e)
  } finally {
    sourcePlaylistLoading.value = false
  }
}

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
        @keyup.enter="searchCurrent()"
      />
      <button class="search__btn" :disabled="loading" @click="searchCurrent()">
        {{ loading ? '搜索中…' : '搜索' }}
      </button>
    </div>

    <div class="search__tools">
      <select v-model="searchType" class="playlist-select">
        <option value="song">歌曲</option>
        <option value="playlist">歌单</option>
      </select>
      <select v-model="source" class="playlist-select">
        <option v-for="item in sourceOptions" :key="item.value" :value="item.value">
          {{ item.label }}
        </option>
      </select>
      <select v-model="selectedPlaylistId" class="playlist-select" :disabled="!canAddToPlaylist">
        <option value="">{{ canAddToPlaylist ? '选择本地歌单' : '请先创建歌单' }}</option>
        <option v-for="item in playlists" :key="item.id" :value="item.id">
          {{ item.name }}
        </option>
      </select>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="actionError" class="error">{{ actionError }}</p>
    <p v-if="actionMessage" class="message">{{ actionMessage }}</p>

    <div v-if="result && searchType === 'song' && result.songs.length" class="results">
      <div class="results__summary">
        共 {{ result.total }} 条结果，当前显示 {{ result.songs.length }} 条
      </div>
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
      <button v-if="result.hasMore" class="load-more" :disabled="loading" @click="search.loadMore()">
        {{ loading ? '加载中…' : '加载更多' }}
      </button>
    </div>

    <div v-else-if="result && searchType === 'playlist' && result.playlists.length" class="results">
      <div class="results__summary">
        共 {{ result.total }} 个音源歌单，当前显示 {{ result.playlists.length }} 个
      </div>
      <div
        v-for="item in result.playlists"
        :key="item.id"
        class="result-item playlist-result"
        :class="{ 'playlist-result--active': selectedSourcePlaylist?.id === item.id }"
        @click="selectSourcePlaylist(item)"
      >
        <div class="result-item__main">
          <div class="result-item__title">{{ item.name }}</div>
          <div class="result-item__meta">
            {{ item.description ?? '暂无描述' }} · {{ item.source }} · {{ item.songCount ?? 0 }} 首
          </div>
        </div>
        <div class="result-item__actions">
          <button class="text-btn" :disabled="sourcePlaylistLoading" @click.stop="selectSourcePlaylist(item)">
            {{ sourcePlaylistLoading && selectedSourcePlaylist?.id === item.id ? '加载中…' : '查看歌曲' }}
          </button>
        </div>
      </div>
      <button v-if="result.hasMore" class="load-more" :disabled="loading" @click="search.loadMore()">
        {{ loading ? '加载中…' : '加载更多' }}
      </button>
    </div>

    <section v-if="selectedSourcePlaylist" class="source-playlist-songs">
      <div class="results__summary">
        {{ selectedSourcePlaylist.name }} · {{ sourcePlaylistSongs.length }} 首歌曲
      </div>
      <div v-if="sourcePlaylistSongs.length" class="results">
        <div v-for="song in sourcePlaylistSongs" :key="song.id" class="result-item">
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
      <div v-else class="empty source-playlist-songs__empty">
        {{ sourcePlaylistLoading ? '正在加载音源歌单歌曲…' : '该音源歌单暂无歌曲' }}
      </div>
    </section>

    <div v-else-if="result" class="empty">
      {{ searchType === 'playlist' ? '暂无匹配音源歌单' : '暂无匹配歌曲' }}
    </div>

    <div v-else class="empty">输入关键词开始搜索，试试 SoundHelix、demo、长青 或 lx</div>
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

.results__summary {
  padding: 0 4px 8px;
  color: var(--text-tertiary);
  font-size: 12px;
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

.result-item:hover,
.playlist-result--active {
  background: var(--bg-secondary);
}

.playlist-result {
  cursor: pointer;
}

.source-playlist-songs {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-top: 8px;
  border-top: 1px solid var(--bg-secondary);
}

.source-playlist-songs__empty {
  margin-top: 24px;
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

.load-more {
  align-self: center;
  margin-top: 12px;
  padding: 8px 18px;
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-secondary);
  font-size: 13px;
}

.load-more:hover:not(:disabled) {
  color: var(--text-primary);
}

.load-more:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.empty {
  margin-top: 60px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 14px;
}
</style>
