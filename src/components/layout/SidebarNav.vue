<script setup lang="ts">
import { RouterLink } from 'vue-router'

const navItems = [
  { to: '/', label: '首页', icon: '⌂' },
  { to: '/search', label: '搜索', icon: '⌕' },
  { to: '/playlists', label: '歌单', icon: '▦' },
  { to: '/local', label: '本地', icon: '♪' },
  { to: '/recent', label: '最近', icon: '↺' },
]

const bottomItems = [{ to: '/settings', label: '设置', icon: '⚙' }]
</script>

<template>
  <nav class="sidebar">
    <div class="sidebar__group">
      <RouterLink
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        class="nav-item"
        active-class="nav-item--active"
      >
        <span class="nav-item__icon">{{ item.icon }}</span>
        <span class="nav-item__label">{{ item.label }}</span>
      </RouterLink>
    </div>

    <div class="sidebar__spacer" />

    <div class="sidebar__group">
      <RouterLink
        v-for="item in bottomItems"
        :key="item.to"
        :to="item.to"
        class="nav-item"
        active-class="nav-item--active"
      >
        <span class="nav-item__icon">{{ item.icon }}</span>
        <span class="nav-item__label">{{ item.label }}</span>
      </RouterLink>
    </div>
  </nav>
</template>

<style scoped>
.sidebar {
  width: 200px;
  flex-shrink: 0;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
  padding: 16px 12px;
  border-right: 1px solid rgba(255, 255, 255, 0.04);
}

.sidebar__group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sidebar__spacer {
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  font-size: 14px;
  transition: all 0.15s ease;
}

.nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item--active {
  background: linear-gradient(
    90deg,
    rgba(124, 108, 240, 0.18),
    rgba(124, 108, 240, 0.06)
  );
  color: var(--text-primary);
}

.nav-item--active .nav-item__icon {
  color: var(--accent);
}

.nav-item__icon {
  font-size: 16px;
  width: 18px;
  text-align: center;
}
</style>
