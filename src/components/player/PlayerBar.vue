<script setup lang="ts">
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { usePlayerStore } from '@/stores/player'

const player = usePlayerStore()
const { currentSong, isPlaying, position, duration, volume, playMode, queue, currentIndex, lyrics, hasPrevious, hasNext } =
  storeToRefs(player)
const queueOpen = ref(false)

function fmt(sec: number): string {
  if (!Number.isFinite(sec) || sec < 0) return '0:00'
  const m = Math.floor(sec / 60)
  const s = Math.floor(sec % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}

const progressPercent = computed(() =>
  duration.value && duration.value > 0
    ? (position.value / duration.value) * 100
    : 0,
)

function onSeek(e: Event) {
  const target = e.target as HTMLInputElement
  const pct = Number(target.value) / 100
  if (duration.value) player.seek(pct * duration.value)
}

function onVolume(e: Event) {
  const target = e.target as HTMLInputElement
  player.setVolume(Number(target.value) / 100)
}

async function playQueueItem(index: number) {
  await player.playQueueSong(index)
  queueOpen.value = false
}

function removeQueueItem(index: number) {
  player.removeFromQueue(index)
  if (!queue.value.length) {
    queueOpen.value = false
  }
}

function clearQueue() {
  player.clearQueue()
  queueOpen.value = false
}

const lyricPreview = computed(() =>
  lyrics.value
    ?.split('\n')
    .map((line) => line.trim())
    .filter(Boolean)
    .slice(0, 2)
    .join('\n') ?? null,
)

const modeLabel = computed(() => {
  switch (playMode.value) {
    case 'sequential':
      return '顺序'
    case 'loop':
      return '循环'
    case 'shuffle':
      return '随机'
    case 'single':
      return '单曲'
  }
})
</script>

<template>
  <footer class="player-bar">
    <div v-if="queueOpen" class="queue-panel">
      <div class="queue-panel__header">
        <div>
          <h2>播放队列</h2>
          <p>{{ queue.length }} 首歌曲</p>
        </div>
        <button class="queue-panel__clear" :disabled="!queue.length" @click="clearQueue">清空</button>
      </div>
      <div v-if="queue.length" class="queue-panel__list">
        <div
          v-for="(song, index) in queue"
          :key="song.id"
          class="queue-item"
          :class="{ 'queue-item--active': index === currentIndex }"
        >
          <button class="queue-item__main" @click="playQueueItem(index)">
            <span class="queue-item__title">{{ song.title }}</span>
            <span class="queue-item__meta">{{ song.artist ?? '未知艺人' }} · {{ song.source }}</span>
          </button>
          <button class="queue-item__action" @click="removeQueueItem(index)">移除</button>
        </div>
      </div>
      <div v-else class="queue-panel__empty">队列为空</div>
    </div>

    <div class="player-bar__info">
      <div class="cover" :class="{ 'cover--spin': isPlaying }">
        <img v-if="currentSong?.coverUrl" :src="currentSong.coverUrl" alt="" />
        <span v-else class="cover__placeholder">♪</span>
      </div>
      <div class="meta">
        <div class="meta__title">
          {{ currentSong?.title ?? '未在播放' }}
        </div>
        <div class="meta__artist">
          {{ currentSong?.artist ?? '—' }}
        </div>
        <div v-if="lyricPreview" class="meta__lyrics">
          {{ lyricPreview }}
        </div>
      </div>
    </div>

    <div class="player-bar__center">
      <div class="controls">
        <button class="ctrl" title="播放模式" @click="player.cyclePlayMode()">
          {{ modeLabel }}
        </button>
        <button class="ctrl" title="上一首" :disabled="!hasPrevious" @click="player.playPrevious()">⏮</button>
        <button class="ctrl ctrl--play" :title="isPlaying ? '暂停' : '播放'" @click="player.togglePause()">
          {{ isPlaying ? '⏸' : '▶' }}
        </button>
        <button class="ctrl" title="下一首" :disabled="!hasNext" @click="player.playNext()">⏭</button>
        <button class="ctrl" title="停止" @click="player.stop()">⏹</button>
      </div>
      <div class="progress">
        <span class="progress__time">{{ fmt(position) }}</span>
        <input
          class="progress__bar"
          type="range"
          min="0"
          max="100"
          step="0.1"
          :value="progressPercent"
          @input="onSeek"
        />
        <span class="progress__time">{{ fmt(duration ?? 0) }}</span>
      </div>
    </div>

    <div class="player-bar__right">
      <button class="queue-count" :class="{ 'queue-count--active': queueOpen }" @click="queueOpen = !queueOpen">
        队列 {{ queue.length }}
      </button>
      <span class="vol-icon">🔊</span>
      <input
        class="vol-bar"
        type="range"
        min="0"
        max="100"
        :value="volume * 100"
        @input="onVolume"
      />
    </div>
  </footer>
</template>

<style scoped>
.player-bar {
  position: relative;
  height: 84px;
  flex-shrink: 0;
  background: var(--player-bg);
  border-top: 1px solid rgba(255, 255, 255, 0.05);
  display: grid;
  grid-template-columns: 1fr 2fr 1fr;
  align-items: center;
  padding: 0 24px;
  gap: 16px;
}

.queue-panel {
  position: absolute;
  right: 24px;
  bottom: 96px;
  width: min(420px, calc(100vw - 48px));
  max-height: 420px;
  display: flex;
  flex-direction: column;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  box-shadow: 0 18px 40px rgba(0, 0, 0, 0.32);
  overflow: hidden;
  z-index: 20;
}

.queue-panel__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 14px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.queue-panel__header h2 {
  font-size: 15px;
  font-weight: 700;
}

.queue-panel__header p,
.queue-panel__empty {
  margin-top: 4px;
  color: var(--text-tertiary);
  font-size: 12px;
}

.queue-panel__clear,
.queue-item__action {
  flex-shrink: 0;
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 12px;
}

.queue-panel__clear:hover:not(:disabled),
.queue-item__action:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.queue-panel__clear:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.queue-panel__list {
  overflow-y: auto;
  padding: 6px;
}

.queue-panel__empty {
  padding: 28px 16px;
  text-align: center;
}

.queue-item {
  display: flex;
  align-items: center;
  gap: 8px;
  border-radius: var(--radius-md);
}

.queue-item--active,
.queue-item:hover {
  background: var(--bg-tertiary);
}

.queue-item__main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
  padding: 10px;
  text-align: left;
}

.queue-item__title,
.queue-item__meta {
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.queue-item__title {
  color: var(--text-primary);
  font-size: 13px;
  font-weight: 600;
}

.queue-item__meta {
  color: var(--text-secondary);
  font-size: 12px;
}

.player-bar__info {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.cover {
  width: 52px;
  height: 52px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  flex-shrink: 0;
}

.cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.cover--spin {
  animation: spin 20s linear infinite;
  border-radius: var(--radius-full);
}

.cover__placeholder {
  font-size: 22px;
  color: var(--text-tertiary);
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.meta {
  min-width: 0;
}

.meta__title {
  font-size: 14px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta__artist,
.meta__lyrics {
  font-size: 12px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta__lyrics {
  max-width: 260px;
  color: var(--text-tertiary);
  white-space: pre-line;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.player-bar__center {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
}

.ctrl {
  color: var(--text-secondary);
  font-size: 14px;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  transition: color 0.15s ease;
}

.ctrl:hover {
  color: var(--text-primary);
}

.ctrl:disabled {
  cursor: not-allowed;
  opacity: 0.4;
}

.ctrl--play {
  width: 38px;
  height: 38px;
  border-radius: var(--radius-full);
  background: linear-gradient(135deg, var(--accent-start), var(--accent-end));
  color: #fff;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.ctrl--play:hover {
  color: #fff;
  filter: brightness(1.1);
}

.progress {
  display: flex;
  align-items: center;
  gap: 10px;
}

.progress__time {
  font-size: 11px;
  color: var(--text-tertiary);
  width: 38px;
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.progress__bar,
.vol-bar {
  -webkit-appearance: none;
  appearance: none;
  height: 4px;
  border-radius: var(--radius-full);
  background: var(--progress-bg);
  outline: none;
  cursor: pointer;
}

.progress__bar {
  flex: 1;
}

.progress__bar::-webkit-slider-thumb,
.vol-bar::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 12px;
  height: 12px;
  border-radius: var(--radius-full);
  background: var(--accent);
}

.player-bar__right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 10px;
}

.vol-icon {
  font-size: 14px;
}

.queue-count {
  padding: 6px 10px;
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  font-size: 12px;
  white-space: nowrap;
}

.queue-count:hover,
.queue-count--active {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.vol-bar {
  width: 90px;
}
</style>
