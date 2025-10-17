<script setup lang="ts">
import { computed } from 'vue'
import VueMarkdownRender from 'vue-markdown-render'

const props = defineProps<{
  content: string
}>()

const hasContent = computed(() => {
  return props.content.trim().length > 0
})
</script>

<template>
  <div class="markdown-preview">
    <div class="preview-header">
      <h3>Live Preview</h3>
    </div>
    <div class="preview-content">
      <div v-if="!hasContent" class="empty-preview">
        <p>Vorschau erscheint hier während Sie tippen...</p>
      </div>
      <VueMarkdownRender 
        v-else
        :source="content"
        class="markdown-content"
      />
    </div>
  </div>
</template>

<style scoped>
.markdown-preview {
  display: flex;
  flex-direction: column;
  height: 100%;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--color-bg-primary);
}

.preview-header {
  padding: 12px 16px;
  background-color: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
}

.preview-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.preview-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  font-size: 14px;
  line-height: 1.6;
  color: var(--color-text-primary);
}

/* Markdown content styling */
.markdown-content :deep(h1) {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 24px 0 16px 0;
  padding-bottom: 8px;
  border-bottom: 2px solid var(--color-border);
}

.markdown-content :deep(h2) {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 20px 0 12px 0;
  padding-bottom: 6px;
  border-bottom: 1px solid var(--color-border);
}

.markdown-content :deep(h3) {
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 16px 0 8px 0;
}

.markdown-content :deep(h4) {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 12px 0 6px 0;
}

.markdown-content :deep(h5),
.markdown-content :deep(h6) {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 10px 0 4px 0;
}

.markdown-content :deep(p) {
  margin: 8px 0;
  color: var(--color-text-primary);
}

.markdown-content :deep(strong) {
  font-weight: 600;
  color: var(--color-text-primary);
}

.markdown-content :deep(em) {
  font-style: italic;
  color: var(--color-text-primary);
}

.markdown-content :deep(code) {
  background-color: var(--color-bg-tertiary);
  color: var(--color-accent);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
}

.markdown-content :deep(pre) {
  background-color: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: 6px;
  padding: 16px;
  margin: 12px 0;
  overflow-x: auto;
}

.markdown-content :deep(pre code) {
  background: none;
  color: var(--color-text-primary);
  padding: 0;
  border-radius: 0;
  font-size: 13px;
}

.markdown-content :deep(blockquote) {
  border-left: 4px solid var(--color-accent);
  background-color: var(--color-bg-secondary);
  margin: 12px 0;
  padding: 12px 16px;
  color: var(--color-text-secondary);
  font-style: italic;
}

.markdown-content :deep(ul),
.markdown-content :deep(ol) {
  margin: 8px 0;
  padding-left: 24px;
}

.markdown-content :deep(li) {
  margin: 4px 0;
  color: var(--color-text-primary);
}

.markdown-content :deep(a) {
  color: var(--color-accent);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.2s ease;
}

.markdown-content :deep(a:hover) {
  border-bottom-color: var(--color-accent);
}

.markdown-content :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin: 12px 0;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  overflow: hidden;
}

.markdown-content :deep(th),
.markdown-content :deep(td) {
  padding: 8px 12px;
  text-align: left;
  border-bottom: 1px solid var(--color-border);
}

.markdown-content :deep(th) {
  background-color: var(--color-bg-secondary);
  font-weight: 600;
  color: var(--color-text-primary);
}

.markdown-content :deep(td) {
  color: var(--color-text-primary);
}

.markdown-content :deep(hr) {
  border: none;
  height: 1px;
  background-color: var(--color-border);
  margin: 20px 0;
}

/* Empty state */
.empty-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text-tertiary);
  font-style: italic;
}

@media (max-width: 768px) {
  .preview-content {
    padding: 12px;
    font-size: 13px;
  }
  
  .markdown-content :deep(h1) {
    font-size: 20px;
  }
  
  .markdown-content :deep(h2) {
    font-size: 18px;
  }
  
  .markdown-content :deep(h3) {
    font-size: 16px;
  }
}
</style>
