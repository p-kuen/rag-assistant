<script setup lang="ts">
import { ref } from 'vue'
import type { DocumentMetadata } from '../types'

const emit = defineEmits<{
  upload: [file: File, metadata: DocumentMetadata]
}>()

const isDragging = ref(false)
const selectedFile = ref<File | null>(null)
const tags = ref('')
const documentType = ref('')
const documentDate = ref('')

const handleDragOver = (event: DragEvent) => {
  event.preventDefault()
  isDragging.value = true
}

const handleDragLeave = () => {
  isDragging.value = false
}

const handleDrop = (event: DragEvent) => {
  event.preventDefault()
  isDragging.value = false

  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    handleFileSelect(files[0])
  }
}

const handleFileInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    handleFileSelect(target.files[0])
  }
}

const handleFileSelect = (file: File) => {
  if (file.name.endsWith('.md') || file.name.endsWith('.pdf')) {
    selectedFile.value = file
  } else {
    alert('Please select a .md or .pdf file')
  }
}

const uploadDocument = () => {
  if (!selectedFile.value) return

  const metadata: DocumentMetadata = {
    tags: tags.value ? tags.value.split(',').map(t => t.trim()) : undefined,
    type: documentType.value || undefined,
    date: documentDate.value || undefined
  }

  emit('upload', selectedFile.value, metadata)

  selectedFile.value = null
  tags.value = ''
  documentType.value = ''
  documentDate.value = ''
}
</script>

<template>
  <div class="upload-container">
    <div
      class="drop-zone"
      :class="{ 'dragging': isDragging, 'has-file': selectedFile }"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
    >
      <input
        type="file"
        id="file-input"
        accept=".md,.pdf"
        @change="handleFileInput"
        style="display: none"
      />

      <div v-if="!selectedFile" class="drop-zone-content">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="17 8 12 3 7 8"/>
          <line x1="12" y1="3" x2="12" y2="15"/>
        </svg>
        <p class="drop-text">Drag and drop your document here</p>
        <p class="drop-subtext">or</p>
        <label for="file-input" class="browse-button">
          Browse files
        </label>
        <p class="file-types">Supported: .md, .pdf</p>
      </div>

      <div v-else class="selected-file">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
          <polyline points="13 2 13 9 20 9"/>
        </svg>
        <div class="file-info">
          <p class="file-name">{{ selectedFile.name }}</p>
          <p class="file-size">{{ (selectedFile.size / 1024).toFixed(2) }} KB</p>
        </div>
        <button @click="selectedFile = null" class="remove-button">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    </div>

    <div v-if="selectedFile" class="metadata-form">
      <div class="form-group">
        <label for="tags">Tags (comma-separated)</label>
        <input
          v-model="tags"
          id="tags"
          type="text"
          placeholder="e.g., architecture, api, guide"
        />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label for="type">Document Type</label>
          <input
            v-model="documentType"
            id="type"
            type="text"
            placeholder="e.g., Guide, Reference"
          />
        </div>
        <div class="form-group">
          <label for="date">Date</label>
          <input
            v-model="documentDate"
            id="date"
            type="date"
          />
        </div>
      </div>
      <button @click="uploadDocument" class="upload-button">
        Upload and Index
      </button>
    </div>
  </div>
</template>

<style scoped>
.upload-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.drop-zone {
  border: 2px dashed var(--color-border);
  border-radius: 12px;
  padding: 48px 24px;
  text-align: center;
  transition: all 0.3s ease;
  background-color: var(--color-bg-secondary);
}

.drop-zone.dragging {
  border-color: var(--color-accent);
  background-color: var(--color-bg-tertiary);
  transform: scale(1.02);
}

.drop-zone.has-file {
  border-style: solid;
  padding: 24px;
}

.drop-zone-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--color-text-secondary);
}

.drop-zone-content svg {
  opacity: 0.5;
}

.drop-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.drop-subtext {
  font-size: 14px;
}

.browse-button {
  display: inline-block;
  padding: 8px 24px;
  background-color: var(--color-accent);
  color: white;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.browse-button:hover {
  background-color: var(--color-accent-hover);
}

.file-types {
  font-size: 12px;
  margin-top: 8px;
}

.selected-file {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background-color: var(--color-bg-primary);
  border-radius: 8px;
}

.selected-file svg {
  flex-shrink: 0;
  color: var(--color-accent);
}

.file-info {
  flex: 1;
  text-align: left;
  min-width: 0;
}

.file-name {
  font-weight: 500;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-size {
  font-size: 14px;
  color: var(--color-text-secondary);
  margin-top: 4px;
}

.remove-button {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: none;
  color: var(--color-text-secondary);
  border-radius: 6px;
  transition: all 0.2s ease;
}

.remove-button:hover {
  background-color: var(--color-bg-tertiary);
  color: var(--color-danger);
}

.metadata-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  background-color: var(--color-bg-secondary);
  border-radius: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.form-group label {
  font-size: 14px;
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
}

.form-group input:focus {
  outline: none;
  border-color: var(--color-accent);
}

.upload-button {
  padding: 12px 24px;
  background-color: var(--color-accent);
  color: white;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  font-size: 15px;
  transition: background-color 0.2s ease;
}

.upload-button:hover {
  background-color: var(--color-accent-hover);
}

@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
}
</style>
