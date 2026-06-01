<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as api from '@/api/commands'

const greeting = ref('')

onMounted(async () => {
  try {
    greeting.value = await api.greet('用户')
  } catch (e) {
    greeting.value = `后端连通失败: ${e}`
  }
})
</script>

<template>
  <div class="home">
    <header class="home__hero">
      <h1 class="home__title">Cmmuu Music</h1>
      <p class="home__subtitle">基于 Tauri v2 + Vue 3 的跨平台音乐播放器</p>
    </header>

    <section class="card">
      <h2 class="card__title">后端连通性</h2>
      <p class="card__text">{{ greeting || '正在连接…' }}</p>
    </section>

    <section class="card">
      <h2 class="card__title">快速开始</h2>
      <p class="card__text">
        前往「本地」页选择音频文件即可试听，验证音频引擎（Symphonia + cpal）。
      </p>
    </section>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.home__hero {
  padding: 12px 0 4px;
}

.home__title {
  font-size: 28px;
  font-weight: 700;
  background: linear-gradient(90deg, var(--accent-start), var(--accent-end));
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.home__subtitle {
  margin-top: 6px;
  color: var(--text-secondary);
  font-size: 14px;
}

.card {
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  padding: 20px;
}

.card__title {
  font-size: 15px;
  font-weight: 600;
  margin-bottom: 8px;
}

.card__text {
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1.6;
}
</style>
