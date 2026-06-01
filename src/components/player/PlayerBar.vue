<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { usePlayerStore } from '@/stores/player'

const player = usePlayerStore()
const { currentSong, isPlaying, position, duration, volume, playMode, queue, lyrics, hasPrevious, hasNext } =
  storeToRefs(player)

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
        <div v-if="lyrics" class="meta__lyrics">
          {{ lyrics }}
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
      <span class="queue-count">队列 {{ queue.length }}</span>
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
  color: var(--text-tertiary);
  font-size: 12px;
  white-space: nowrap;
}

.vol-bar {
  width: 90px;
}
</style>
