<script setup lang="ts">
import { computed } from 'vue'
import type { UploadTask } from '../types'

const props = defineProps<{
  tasks: UploadTask[]
}>()

const statistics = computed(() => {
  const total = props.tasks.length
  const processing = props.tasks.filter(t => t.status === 'processing').length
  const succeeded = props.tasks.filter(t => t.status === 'succeeded').length
  const failed = props.tasks.filter(t => t.status === 'failed').length
  
  return {
    total,
    processing,
    succeeded,
    failed,
    successRate: total > 0 ? Math.round((succeeded / total) * 100) : 0
  }
})
</script>

<template>
  <div class="task-status-container">
    <div class="task-header">
      <h2>Indexierungs-Status</h2>
      <div class="task-count">{{ statistics.total }} insgesamt</div>
    </div>

    <div v-if="statistics.total > 0" class="statistics-grid">
      <div class="stat-card">
        <div class="stat-value">{{ statistics.succeeded }}</div>
        <div class="stat-label">Erfolgreich</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ statistics.processing }}</div>
        <div class="stat-label">In Bearbeitung</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ statistics.failed }}</div>
        <div class="stat-label">Fehlgeschlagen</div>
      </div>
      <div class="stat-card">
        <div class="stat-value">{{ statistics.successRate }}%</div>
        <div class="stat-label">Erfolgsrate</div>
      </div>
    </div>

    <div v-if="tasks.length === 0" class="empty-tasks">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <polyline points="9 11 12 14 22 4"/>
        <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/>
      </svg>
      <p>Noch keine Indexierungsaufgaben</p>
    </div>

    <div v-else class="task-table">
      <div class="table-header">
        <div class="col-status">Status</div>
        <div class="col-filename">Dateiname</div>
        <div class="col-date">Hochgeladen</div>
      </div>
      <div class="table-body">
        <div
          v-for="task in tasks"
          :key="task.id"
          class="table-row"
        >
          <div class="col-status">
            <span
              class="status-badge"
              :class="task.status"
            >
              <span class="status-icon" v-if="task.status === 'processing'">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                </svg>
              </span>
              <span class="status-icon" v-else-if="task.status === 'succeeded'">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              </span>
              <span class="status-icon" v-else>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <line x1="18" y1="6" x2="6" y2="18"/>
                  <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
              </span>
              {{ task.status === 'processing' ? 'Verarbeitung' : task.status === 'succeeded' ? 'Erfolgreich' : 'Fehlgeschlagen' }}
            </span>
          </div>
          <div class="col-filename">
            <span class="filename">{{ task.filename }}</span>
            <span v-if="task.error" class="error-text">{{ task.error }}</span>
          </div>
          <div class="col-date">
            {{ new Date(task.uploadedAt).toLocaleString() }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.task-status-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.task-header h2 {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.task-count {
  padding: 4px 12px;
  background-color: var(--color-bg-tertiary);
  border-radius: 12px;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.statistics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background-color: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 16px;
  text-align: center;
  transition: all 0.2s ease;
}

.stat-card:hover {
  background-color: var(--color-bg-tertiary);
  border-color: var(--color-accent);
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-accent);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.empty-tasks {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 64px 24px;
  color: var(--color-text-secondary);
  text-align: center;
}

.empty-tasks svg {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-tasks p {
  font-size: 16px;
}

.task-table {
  background-color: var(--color-bg-secondary);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--shadow-sm);
}

.table-header {
  display: grid;
  grid-template-columns: 150px 1fr 200px;
  padding: 16px 24px;
  background-color: var(--color-bg-tertiary);
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.table-body {
  display: flex;
  flex-direction: column;
}

.table-row {
  display: grid;
  grid-template-columns: 150px 1fr 200px;
  padding: 16px 24px;
  border-bottom: 1px solid var(--color-border);
  transition: background-color 0.2s ease;
}

.table-row:last-child {
  border-bottom: none;
}

.table-row:hover {
  background-color: var(--color-bg-tertiary);
}

.col-status {
  display: flex;
  align-items: center;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  text-transform: capitalize;
}

.status-badge.processing {
  background-color: rgba(255, 193, 7, 0.1);
  color: var(--color-warning);
}

.status-badge.succeeded {
  background-color: rgba(40, 167, 69, 0.1);
  color: var(--color-success);
}

.status-badge.failed {
  background-color: rgba(220, 53, 69, 0.1);
  color: var(--color-danger);
}

.status-icon {
  display: flex;
  align-items: center;
}

.status-badge.processing .status-icon svg {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.col-filename {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.filename {
  font-weight: 500;
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.error-text {
  font-size: 12px;
  color: var(--color-danger);
}

.col-date {
  display: flex;
  align-items: center;
  font-size: 14px;
  color: var(--color-text-secondary);
}

@media (max-width: 1024px) {
  .table-header,
  .table-row {
    grid-template-columns: 130px 1fr 150px;
    padding: 12px 16px;
  }

  .col-date {
    font-size: 12px;
  }
}

@media (max-width: 768px) {
  .table-header {
    display: none;
  }

  .table-row {
    grid-template-columns: 1fr;
    gap: 12px;
    padding: 16px;
  }

  .col-status,
  .col-filename,
  .col-date {
    grid-column: 1;
  }

  .col-date::before {
    content: 'Hochgeladen: ';
    font-weight: 500;
  }

  .statistics-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }

  .stat-card {
    padding: 12px;
  }

  .stat-value {
    font-size: 20px;
  }
}
</style>
