<script setup lang="ts">
import { ref } from 'vue'
import type { Message } from '../types'
import SourceAttribution from './SourceAttribution.vue'

defineProps<{
  message: Message
}>()

const showSources = ref(false)
</script>

<template>
  <div class="message-wrapper" :class="{ 'user-message': message.type === 'user' }">
    <div class="message-bubble">
      <div class="message-content">{{ message.content }}</div>
      <div v-if="message.sources && message.sources.length > 0" class="sources-section">
        <button
          @click="showSources = !showSources"
          class="sources-toggle"
        >
          {{ showSources ? 'Hide Sources' : `Show Sources (${message.sources.length})` }}
        </button>
        <SourceAttribution v-if="showSources" :sources="message.sources" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.message-wrapper {
  display: flex;
  margin-bottom: 16px;
  animation: slideIn 0.3s ease;
}

.message-wrapper.user-message {
  justify-content: flex-end;
}

.message-bubble {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: 12px;
  background-color: var(--color-bg-secondary);
  box-shadow: var(--shadow-sm);
}

.user-message .message-bubble {
  background-color: var(--color-accent);
  color: white;
}

.message-content {
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}

.sources-section {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--color-border);
}

.user-message .sources-section {
  border-top-color: rgba(255, 255, 255, 0.2);
}

.sources-toggle {
  background: none;
  border: none;
  color: var(--color-accent);
  font-size: 14px;
  font-weight: 500;
  padding: 4px 0;
  text-decoration: underline;
}

.user-message .sources-toggle {
  color: white;
}

.sources-toggle:hover {
  opacity: 0.8;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 768px) {
  .message-bubble {
    max-width: 85%;
  }
}
</style>
