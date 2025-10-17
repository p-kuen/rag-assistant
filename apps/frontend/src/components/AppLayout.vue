<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { MessageSquare, Database, Menu, X } from 'lucide-vue-next'
import { ref } from 'vue'

const router = useRouter()
const route = useRoute()
const mobileMenuOpen = ref(false)

const navigation = [
  { name: 'Chat-Assistent', path: '/chat', icon: MessageSquare },
  { name: 'Wissensverwaltung', path: '/admin/knowledge', icon: Database }
]

const isActivePath = (path: string) => {
  return route.path === path
}

const navigateTo = (path: string) => {
  router.push(path)
  mobileMenuOpen.value = false
}

const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}
</script>

<template>
  <div class="h-screen flex flex-col bg-white dark:bg-gray-950">
    <!-- Header -->
    <header class="bg-white dark:bg-gray-900 border-b border-gray-300 dark:border-gray-700 px-6 py-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 bg-accent rounded-lg flex items-center justify-center">
          <span class="text-white font-bold text-lg">R</span>
        </div>
        <h1 class="text-xl font-bold text-gray-900 dark:text-gray-100">
          RAG Assistant
        </h1>
      </div>

      <!-- Desktop Navigation -->
      <nav class="hidden md:flex gap-2">
        <button
          v-for="item in navigation"
          :key="item.path"
          @click="navigateTo(item.path)"
          :class="[
            'flex items-center gap-2 px-4 py-2 rounded-lg transition-colors',
            isActivePath(item.path)
              ? 'bg-accent text-white'
              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-800'
          ]"
        >
          <component :is="item.icon" :size="18" />
          <span class="text-sm font-medium">{{ item.name }}</span>
        </button>
      </nav>

      <!-- Mobile Menu Button -->
      <button
        @click="toggleMobileMenu"
        class="md:hidden p-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-800 rounded-lg"
      >
        <Menu v-if="!mobileMenuOpen" :size="24" />
        <X v-else :size="24" />
      </button>
    </header>

    <!-- Mobile Navigation -->
    <div
      v-if="mobileMenuOpen"
      class="md:hidden bg-white dark:bg-gray-900 border-b border-gray-300 dark:border-gray-700"
    >
      <nav class="flex flex-col p-4 gap-2">
        <button
          v-for="item in navigation"
          :key="item.path"
          @click="navigateTo(item.path)"
          :class="[
            'flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-left',
            isActivePath(item.path)
              ? 'bg-accent text-white'
              : 'text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-800'
          ]"
        >
          <component :is="item.icon" :size="20" />
          <span class="font-medium">{{ item.name }}</span>
        </button>
      </nav>
    </div>

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden">
      <slot />
    </main>
  </div>
</template>
