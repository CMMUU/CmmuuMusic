<script setup lang="ts">
// 最近播放（占位）：播放历史功能将在播放列表/历史阶段接入 play_history 表。
</script>

<template>
  <div class="recent">
    <h1>最近播放</h1>
    <div class="empty">播放历史将在后续阶段接入</div>
  </div>
</template>

<style scoped>
.recent h1 {
  font-size: 22px;
  font-weight: 700;
  margin-bottom: 16px;
}

.empty {
  margin-top: 60px;
  text-align: center;
  color: var(--text-tertiary);
  font-size: 14px;
}
</style>
