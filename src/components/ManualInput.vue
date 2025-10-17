<script setup lang="ts">
import { ref } from 'vue'
import MarkdownEditor from './MarkdownEditor.vue'
import MarkdownPreview from './MarkdownPreview.vue'
import type { DocumentMetadata } from '../types'

const emit = defineEmits<{
  'save-content': [content: string, metadata: DocumentMetadata]
}>()

const markdownContent = ref('')
const tags = ref('')
const documentType = ref('')
const documentDate = ref('')

const handleContentUpdate = (content: string) => {
  markdownContent.value = content
}

const saveContent = () => {
  if (!markdownContent.value.trim()) {
    alert('Please enter some content before saving.')
    return
  }

  const metadata: DocumentMetadata = {
    tags: tags.value ? tags.value.split(',').map(t => t.trim()) : undefined,
    type: documentType.value || undefined,
    date: documentDate.value || undefined
  }

  emit('save-content', markdownContent.value, metadata)

  // Reset form
  markdownContent.value = ''
  tags.value = ''
  documentType.value = ''
  documentDate.value = ''
}

const clearContent = () => {
  markdownContent.value = ''
  tags.value = ''
  documentType.value = ''
  documentDate.value = ''
}
</script>

<template>
  <div class="manual-input">
    <div class="input-header">
      <h2>Manuelle Eingabe</h2>
      <p>Erstellen Sie Inhalte direkt mit dem Markdown-Editor</p>
    </div>

    <div class="split-screen">
      <div class="editor-panel">
        <MarkdownEditor 
          :content="markdownContent"
          @update:content="handleContentUpdate"
          placeholder="Geben Sie hier Ihr Markdown ein..."
        />
      </div>
      
      <div class="preview-panel">
        <MarkdownPreview :content="markdownContent" />
      </div>
    </div>

    <div class="metadata-section">
      <h3>Metadaten</h3>
      <div class="metadata-form">
        <div class="form-group">
          <label for="manual-tags">Tags (durch Komma getrennt)</label>
          <input
            v-model="tags"
            id="manual-tags"
            type="text"
            placeholder="z.B. Architektur, API, Anleitung"
          />
        </div>
        <div class="form-row">
          <div class="form-group">
            <label for="manual-type">Dokumenttyp</label>
            <input
              v-model="documentType"
              id="manual-type"
              type="text"
              placeholder="z.B. Anleitung, Referenz"
            />
          </div>
          <div class="form-group">
            <label for="manual-date">Datum</label>
            <input
              v-model="documentDate"
              id="manual-date"
              type="date"
            />
          </div>
        </div>
      </div>
    </div>

    <div class="action-buttons">
      <button @click="clearContent" class="clear-button">
        Löschen
      </button>
      <button @click="saveContent" class="save-button" :disabled="!markdownContent.trim()">
        Speichern & Indexieren
      </button>
    </div>
  </div>
</template>

<style scoped>
.manual-input {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.input-header {
  text-align: center;
}

.input-header h2 {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 8px;
}

.input-header p {
  font-size: 14px;
  color: var(--color-text-secondary);
}

.split-screen {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  height: 500px;
  min-height: 400px;
}

.editor-panel,
.preview-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.metadata-section {
  background-color: var(--color-bg-secondary);
  border-radius: 12px;
  padding: 20px;
}

.metadata-section h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 16px;
}

.metadata-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.form-group label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.form-group input {
  padding: 10px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-size: 14px;
  transition: border-color 0.2s ease;
}

.form-group input:focus {
  outline: none;
  border-color: var(--color-accent);
}

.form-group input::placeholder {
  color: var(--color-text-tertiary);
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.clear-button,
.save-button {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.clear-button {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.clear-button:hover {
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
}

.save-button {
  background-color: var(--color-accent);
  color: white;
}

.save-button:hover:not(:disabled) {
  background-color: var(--color-accent-hover);
}

.save-button:disabled {
  background-color: var(--color-bg-tertiary);
  color: var(--color-text-tertiary);
  cursor: not-allowed;
}

@media (max-width: 1024px) {
  .split-screen {
    grid-template-columns: 1fr;
    height: auto;
    min-height: 600px;
  }
  
  .editor-panel,
  .preview-panel {
    height: 300px;
  }
}

@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .clear-button,
  .save-button {
    width: 100%;
  }
  
  .input-header h2 {
    font-size: 20px;
  }
}
</style>

