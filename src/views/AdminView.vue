<script setup lang="ts">
import { ref } from 'vue'
import DocumentUpload from '../components/DocumentUpload.vue'
import ManualInput from '../components/ManualInput.vue'
import TaskStatus from '../components/TaskStatus.vue'
import type { UploadTask, DocumentMetadata } from '../types'

const tasks = ref<UploadTask[]>([])
const activeTab = ref<'upload' | 'manual'>('upload')

const handleUpload = async (file: File, metadata: DocumentMetadata) => {
  const taskId = Date.now().toString()

  const newTask: UploadTask = {
    id: taskId,
    filename: file.name,
    status: 'processing',
    uploadedAt: new Date()
  }

  tasks.value.unshift(newTask)

  await simulateUpload(file, metadata, taskId)
}

const handleManualSave = async (content: string, metadata: DocumentMetadata) => {
  const taskId = Date.now().toString()
  const filename = `manual-input-${new Date().toISOString().split('T')[0]}.md`

  const newTask: UploadTask = {
    id: taskId,
    filename: filename,
    status: 'processing',
    uploadedAt: new Date()
  }

  tasks.value.unshift(newTask)

  await simulateManualSave(content, metadata, taskId)
}

const simulateUpload = async (_file: File, _metadata: DocumentMetadata, taskId: string) => {
  await new Promise(resolve => setTimeout(resolve, 2000))

  const task = tasks.value.find(t => t.id === taskId)
  if (task) {
    task.status = 'succeeded'
  }
}

const simulateManualSave = async (_content: string, _metadata: DocumentMetadata, taskId: string) => {
  await new Promise(resolve => setTimeout(resolve, 1500))

  const task = tasks.value.find(t => t.id === taskId)
  if (task) {
    task.status = 'succeeded'
  }
}
</script>

<template>
  <div class="admin-view">
    <div class="admin-container">
      <div class="admin-header">
        <h1>Wissensverwaltung</h1>
        <p>Verwalten Sie Ihr Wissensspeicher durch Upload oder manuelle Eingabe</p>
      </div>

      <div class="admin-content">
        <div class="tab-navigation">
          <button 
            @click="activeTab = 'upload'" 
            :class="{ active: activeTab === 'upload' }"
            class="tab-button"
          >
            📁 Dokument Upload
          </button>
          <button 
            @click="activeTab = 'manual'" 
            :class="{ active: activeTab === 'manual' }"
            class="tab-button"
          >
            ✏️ Manuelle Eingabe
          </button>
        </div>

        <div class="tab-content">
          <section v-if="activeTab === 'upload'" class="upload-section">
            <h2>Dokument Upload</h2>
            <DocumentUpload @upload="handleUpload" />
          </section>

          <section v-if="activeTab === 'manual'" class="manual-section">
            <ManualInput @save-content="handleManualSave" />
          </section>
        </div>

        <section class="status-section">
          <h2>Indexierungs-Status</h2>
          <TaskStatus :tasks="tasks" />
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.admin-view {
  height: 100%;
  overflow-y: auto;
  background-color: var(--color-bg-primary);
}

.admin-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 32px 24px;
}

.admin-header {
  margin-bottom: 40px;
}

.admin-header h1 {
  font-size: 32px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 8px;
}

.admin-header p {
  font-size: 16px;
  color: var(--color-text-secondary);
}

.admin-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.tab-navigation {
  display: flex;
  gap: 8px;
  border-bottom: 1px solid var(--color-border);
  margin-bottom: 24px;
}

.tab-button {
  padding: 12px 20px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--color-text-secondary);
  font-weight: 500;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.tab-button:hover {
  color: var(--color-text-primary);
  background-color: var(--color-bg-tertiary);
}

.tab-button.active {
  color: var(--color-accent);
  border-bottom-color: var(--color-accent);
  background-color: var(--color-bg-secondary);
}

.tab-content {
  min-height: 400px;
}

.upload-section,
.manual-section,
.status-section {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.upload-section h2,
.manual-section h2,
.status-section h2 {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

@media (max-width: 768px) {
  .admin-container {
    padding: 24px 16px;
  }

  .admin-header h1 {
    font-size: 24px;
  }

  .admin-header p {
    font-size: 14px;
  }

  .admin-content {
    gap: 24px;
  }

  .tab-navigation {
    flex-direction: column;
    gap: 0;
  }

  .tab-button {
    border-bottom: 1px solid var(--color-border);
    border-radius: 0;
  }

  .tab-button.active {
    border-bottom-color: var(--color-border);
    border-left: 3px solid var(--color-accent);
  }
}
</style>
