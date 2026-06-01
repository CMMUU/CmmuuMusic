import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import './styles/main.css'

function showStartupError(error: unknown) {
  const message = error instanceof Error ? error.stack || error.message : String(error)
  const appRoot = document.querySelector('#app')

  if (appRoot) {
    appRoot.innerHTML = `
      <div style="padding: 24px; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; color: #f5f5f7; background: #121214; min-height: 100vh;">
        <h1 style="font-size: 20px; margin-bottom: 12px;">Cmmuu Music 启动失败</h1>
        <pre style="white-space: pre-wrap; line-height: 1.5; color: #ff9f0a;">${message}</pre>
      </div>
    `
  }

  console.error('Cmmuu Music 启动失败', error)
}

try {
  const app = createApp(App)

  app.config.errorHandler = (error) => {
    showStartupError(error)
  }

  app.use(createPinia())
  app.use(router)

  app.mount('#app')
} catch (error) {
  showStartupError(error)
}
