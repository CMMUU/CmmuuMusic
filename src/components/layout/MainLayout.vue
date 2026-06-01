<script setup lang="ts">
import { RouterView } from 'vue-router'
import { storeToRefs } from 'pinia'
import { onMounted } from 'vue'
import TitleBar from './TitleBar.vue'
import SidebarNav from './SidebarNav.vue'
import PlayerBar from '@/components/player/PlayerBar.vue'
import { useSettingsStore } from '@/stores/settings'

const settings = useSettingsStore()
const { useCustomTitleBar } = storeToRefs(settings)

onMounted(() => {
  if (!settings.loaded) void settings.load()
})
</script>

<template>
  <div class="app-shell">
    <TitleBar v-if="useCustomTitleBar" />
    <div class="app-body">
      <SidebarNav />
      <main class="app-content">
        <RouterView v-slot="{ Component }">
          <Transition name="fade" mode="out-in">
            <component :is="Component" />
          </Transition>
        </RouterView>
      </main>
    </div>
    <PlayerBar />
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background: var(--bg-primary);
}

.app-body {
  flex: 1;
  display: flex;
  min-height: 0;
}

.app-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px 28px;
  min-width: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
