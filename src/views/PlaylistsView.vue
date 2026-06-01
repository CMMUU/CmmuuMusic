<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { usePlaylistStore } from '@/stores/playlist'
import { usePlayerStore } from '@/stores/player'

const playlist = usePlaylistStore()
const player = usePlayerStore()
const { playlists, loading, selectedSongs, selectedPlaylistId, songsLoading } = storeToRefs(playlist)

const name = ref('')
const error = ref<string | null>(null)

const canCreate = computed(() => name.value.trim().length > 0 && !loading.value)

onMounted(() => {
  void refresh()
})

async function refresh() {
  error.value = null
  try {
    await playlist.refresh()
  } catch (e) {
    error.value = String(e)
  }
}

async function createPlaylist() {
  const nextName = name.value.trim()
  if (!nextName) return

  error.value = null
  try {
    await playlist.create(nextName)
    name.value = ''
  } catch (e) {
    error.value = String(e)
  }
}

async function removePlaylist(id: string) {
  error.value = null
  try {
    await playlist.remove(id)
  } catch (e) {
    error.value = String(e)
  }
}

async function selectPlaylist(id: string) {
  error.value = null
  try {
    await playlist.refreshSongs(id)
  } catch (e) {
    error.value = String(e)
  }
}

function addSelectedSongsToQueue() {
  player.addManyToQueue(selectedSongs.value)
}
</script>

<template>
  <div class="playlists">
    <header class="playlists__header">
      <div>
        <h1>歌单</h1>
        <p>管理本地播放列表，数据持久化到 SQLite。</p>
      </div>
      <button class="ghost-btn" :disabled="loading" @click="refresh">
        {{ loading ? '刷新中…' : '刷新' }}
      </button>
    </header>

    <form class="create-card" @submit.prevent="createPlaylist">
      <input
        v-model="name"
        class="create-card__input"
        type="text"
        placeholder="输入新歌单名称"
      />
      <button class="primary-btn" :disabled="!canCreate" type="submit">创建歌单</button>
    </form>

    <p v-if="error" class="error">{{ error }}</p>

    <div v-if="playlists.length" class="playlist-grid">
      <article
        v-for="item in playlists"
        :key="item.id"
        class="playlist-card"
        :class="{ 'playlist-card--active': selectedPlaylistId === item.id }"
        @click="selectPlaylist(item.id)"
      >
        <div class="playlist-card__cover">♪</div>
        <div class="playlist-card__body">
          <h2>{{ item.name }}</h2>
          <p>{{ selectedPlaylistId === item.id ? selectedSongs.length : item.songs.length }} 首歌曲</p>
          <p class="playlist-card__time">更新于 {{ item.updatedAt }}</p>
        </div>
        <button class="playlist-card__delete" @click.stop="removePlaylist(item.id)">
          删除
        </button>
      </article>
    </div>

    <div v-else-if="!loading" class="empty">
      <p>还没有歌单</p>
      <p class="empty__hint">创建一个歌单后，它会保存到本地数据库。</p>
    </div>
    <section v-if="selectedPlaylistId" class="songs-panel">
      <div class="songs-panel__header">
        <h2>歌单歌曲</h2>
        <button class="text-btn" :disabled="!selectedSongs.length" @click="addSelectedSongsToQueue">
          全部加入队列
        </button>
      </div>
      <div v-if="songsLoading" class="empty empty--small">加载歌曲中…</div>
      <ul v-else-if="selectedSongs.length" class="song-list">
        <li v-for="song in selectedSongs" :key="song.id" class="song-item">
          <div class="song-item__main">
            <span class="song-item__title">{{ song.title }}</span>
            <span class="song-item__meta">{{ song.artist ?? '未知艺人' }} · {{ song.album ?? '未知专辑' }}</span>
          </div>
          <button class="text-btn" @click="player.addToQueue(song)">加入队列</button>
        </li>
      </ul>
      <div v-else class="empty empty--small">这个歌单还没有歌曲</div>
    </section>
  </div>
</template>

<style scoped>
.playlists {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.playlists__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.playlists__header h1 {
  font-size: 22px;
  font-weight: 700;
}

.playlists__header p {
  margin-top: 6px;
  color: var(--text-secondary);
  font-size: 13px;
}

.create-card {
  display: flex;
  gap: 10px;
  padding: 16px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
}

.create-card__input {
  flex: 1;
  min-width: 0;
  padding: 10px 12px;
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 14px;
}

.create-card__input:focus {
  outline: none;
  border-color: var(--accent);
}

.primary-btn,
.ghost-btn {
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 600;
}

.primary-btn {
  background: linear-gradient(135deg, var(--accent-start), var(--accent-end));
  color: #fff;
}

.primary-btn:disabled,
.ghost-btn:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.ghost-btn {
  background: var(--bg-secondary);
  color: var(--text-secondary);
}

.ghost-btn:hover:not(:disabled) {
  color: var(--text-primary);
}

.error {
  color: var(--error);
  font-size: 13px;
  background: rgba(255, 69, 58, 0.1);
  padding: 10px 14px;
  border-radius: var(--radius-md);
}

.playlist-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 12px;
}

.playlist-card {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
  padding: 14px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
}

.playlist-card:hover,
.playlist-card--active {
  outline: 1px solid rgba(124, 108, 240, 0.45);
}

.playlist-card__cover {
  width: 52px;
  height: 52px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--accent);
  font-size: 22px;
}

.playlist-card__body {
  flex: 1;
  min-width: 0;
}

.playlist-card__body h2 {
  font-size: 15px;
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.playlist-card__body p {
  margin-top: 4px;
  color: var(--text-secondary);
  font-size: 12px;
}

.playlist-card__time {
  color: var(--text-tertiary) !important;
}

.playlist-card__delete {
  flex-shrink: 0;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  font-size: 12px;
}

.playlist-card__delete:hover {
  background: rgba(255, 69, 58, 0.1);
  color: var(--error);
}

.empty {
  margin-top: 48px;
  text-align: center;
  color: var(--text-secondary);
}

.empty__hint {
  margin-top: 8px;
  color: var(--text-tertiary);
  font-size: 13px;
}

.empty--small {
  margin-top: 18px;
}

.songs-panel {
  margin-top: 4px;
  padding: 18px 20px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
}

.songs-panel h2 {
  font-size: 15px;
  font-weight: 600;
}

.songs-panel__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 12px;
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

.text-btn:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.song-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.song-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.song-item__main {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.song-item:last-child {
  border-bottom: none;
}

.song-item__title {
  font-size: 14px;
  font-weight: 600;
}

.song-item__meta {
  color: var(--text-secondary);
  font-size: 12px;
}
</style>
