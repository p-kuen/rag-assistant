<script setup lang="ts">
import type { Source } from '../types'

defineProps<{
  sources: Source[]
}>()
</script>

<template>
  <div class="sources-container">
    <div class="sources-header">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
        <path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0zM5.78 8.75a.75.75 0 0 0 0 1.5h4.44a.75.75 0 0 0 0-1.5H5.78zM6.25 5.75a.75.75 0 0 0 0 1.5h3.5a.75.75 0 0 0 0-1.5h-3.5z"/>
      </svg>
      <span>Sources</span>
    </div>
    <div class="sources-list">
      <div
        v-for="(source, index) in sources"
        :key="index"
        class="source-item"
      >
        <div class="source-number">{{ index + 1 }}</div>
        <div class="source-details">
          <div class="source-title">{{ source.title }}</div>
          <div class="source-meta">
            <span class="source-filename">{{ source.filename }}</span>
            <span v-if="source.hierarchyPath" class="source-path"> / {{ source.hierarchyPath }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sources-container {
  margin-top: 8px;
}

.sources-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
}

.sources-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.source-item {
  display: flex;
  gap: 12px;
  padding: 8px;
  background-color: var(--color-bg-tertiary);
  border-radius: 6px;
  font-size: 13px;
  transition: transform 0.2s ease;
}

.source-item:hover {
  transform: translateX(4px);
}

.source-number {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--color-accent);
  color: white;
  border-radius: 50%;
  font-weight: 600;
  font-size: 12px;
}

.source-details {
  flex: 1;
  min-width: 0;
}

.source-title {
  font-weight: 500;
  color: var(--color-text-primary);
  margin-bottom: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.source-meta {
  color: var(--color-text-secondary);
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.source-filename {
  font-weight: 500;
}

.source-path {
  opacity: 0.8;
}
</style>
