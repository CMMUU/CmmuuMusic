<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useSearchStore } from '@/stores/search'

const search = useSearchStore()
const { keyword, result, loading, error } = storeToRefs(search)
</script>

<template>
  <div class="search">
    <div class="search__bar">
      <input
        v-model="keyword"
        class="search__input"
        type="text"
        placeholder="搜索歌曲、专辑、艺人…"
        @keyup.enter="search.search()"
      />
      <button class="search__btn" :disabled="loading" @click="search.search()">
        {{ loading ? '搜索中…' : '搜索' }}
      </button>
    </div>

    <p v-if="error" class="error">{{ error }}</p>

    <div v-if="result && result.songs.length" class="results">
      <div v-for="song in result.songs" :key="song.id" class="result-item">
        {{ song.title }} — {{ song.artist }}
      </div>
    </div>

    <div v-else-if="result" class="empty">
      暂无结果（搜索功能将在插件系统接入后可用）
    </div>

    <div v-else class="empty">输入关键词开始搜索</div>
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

.search__input:focus {
  border-color: var(--accent);
}

.search__btn {
  padding: 0 24px;
  border-radius: var(--radius-md);
  background: linear-gradient(135deg, var(--accent-start), var(--accent-end));
  color: #fff;
  font-weight: 600;
  font-size: 13px;
}

.search__btn:disabled {
  opacity: 0.6;
}

.error {
  color: var(--error);
  font-size: 13px;
}

.result-item {
  padding: 12px 14px;
  border-radius: var(--radius-md);
  font-size: 14px;
}

.result-item:hover {
  background: var(--bg-secondary);
}

.empty {
  margin-top: 60px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 14px;
}
</style>
