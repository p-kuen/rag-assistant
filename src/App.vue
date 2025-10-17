<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed } from 'vue'

const route = useRoute()
const isAdminRoute = computed(() => route.path === '/admin')
</script>

<template>
  <div class="app-container">
    <nav class="main-nav">
      <div class="nav-content">
        <div class="nav-brand">
          <h1>RAG Assistant</h1>
        </div>
        <div class="nav-links">
          <router-link to="/" class="nav-link" :class="{ active: !isAdminRoute }">
            Chat
          </router-link>
          <router-link to="/admin" class="nav-link" :class="{ active: isAdminRoute }">
            Wissensverwaltung
          </router-link>
        </div>
      </div>
    </nav>
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.main-nav {
  background-color: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  position: sticky;
  top: 0;
  z-index: 100;
}

.nav-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 64px;
}

.nav-brand h1 {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.nav-links {
  display: flex;
  gap: 8px;
}

.nav-link {
  padding: 8px 16px;
  border-radius: 6px;
  text-decoration: none;
  color: var(--color-text-secondary);
  font-weight: 500;
  transition: all 0.2s ease;
}

.nav-link:hover {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.nav-link.active {
  background-color: var(--color-accent);
  color: white;
}

.main-content {
  flex: 1;
  overflow: hidden;
}

@media (max-width: 768px) {
  .nav-content {
    padding: 0 16px;
    height: 56px;
  }

  .nav-brand h1 {
    font-size: 18px;
  }

  .nav-link {
    padding: 6px 12px;
    font-size: 14px;
  }
}
</style>
