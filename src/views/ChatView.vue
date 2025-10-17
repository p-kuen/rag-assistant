<script setup lang="ts">
import { ref, nextTick } from 'vue'
import ChatMessage from '../components/ChatMessage.vue'
import type { Message } from '../types'

const messages = ref<Message[]>([])
const userInput = ref('')
const isLoading = ref(false)
const messagesContainer = ref<HTMLElement | null>(null)

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

const sendMessage = async () => {
  const query = userInput.value.trim()
  if (!query || isLoading.value) return

  const userMessage: Message = {
    id: Date.now().toString(),
    type: 'user',
    content: query,
    timestamp: new Date()
  }

  messages.value.push(userMessage)
  userInput.value = ''
  scrollToBottom()

  const assistantMessage: Message = {
    id: (Date.now() + 1).toString(),
    type: 'assistant',
    content: '',
    sources: [],
    timestamp: new Date()
  }

  messages.value.push(assistantMessage)
  isLoading.value = true

  try {
    await simulateStreamingResponse(assistantMessage, query)
  } catch (error) {
    assistantMessage.content = 'Sorry, an error occurred while processing your request.'
  } finally {
    isLoading.value = false
    scrollToBottom()
  }
}

const simulateStreamingResponse = async (message: Message, query: string) => {
  const sampleResponse = `Based on your query about "${query}", here's what I found in the knowledge base. This response demonstrates the streaming capability where tokens appear one by one in real-time. The system uses RAG (Retrieval-Augmented Generation) to provide accurate, context-aware answers based on your documents.`

  const sampleSources = [
    {
      title: 'Introduction to RAG Systems',
      filename: 'rag-overview.md',
      hierarchyPath: 'documentation/architecture',
      score: 0.95
    },
    {
      title: 'Implementation Guide',
      filename: 'implementation.md',
      hierarchyPath: 'documentation/guides',
      score: 0.87
    }
  ]

  const words = sampleResponse.split(' ')
  for (const word of words) {
    message.content += (message.content ? ' ' : '') + word
    scrollToBottom()
    await new Promise(resolve => setTimeout(resolve, 50))
  }

  message.sources = sampleSources
  scrollToBottom()
}

const handleKeyPress = (event: KeyboardEvent) => {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault()
    sendMessage()
  }
}
</script>

<template>
  <div class="chat-view">
    <div class="chat-container">
      <div ref="messagesContainer" class="messages-container">
        <div v-if="messages.length === 0" class="empty-state">
          <div class="empty-icon">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
            </svg>
          </div>
          <h2>Start a conversation</h2>
          <p>Ask questions about your documents and get AI-powered answers with source attribution.</p>
        </div>
        <ChatMessage
          v-for="message in messages"
          :key="message.id"
          :message="message"
        />
      </div>
      <div class="input-container">
        <div class="input-wrapper">
          <textarea
            v-model="userInput"
            @keypress="handleKeyPress"
            placeholder="Ask a question about your documents..."
            rows="1"
            :disabled="isLoading"
            class="chat-input"
          />
          <button
            @click="sendMessage"
            :disabled="!userInput.trim() || isLoading"
            class="send-button"
          >
            <svg v-if="!isLoading" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="22" y1="2" x2="11" y2="13"/>
              <polygon points="22 2 15 22 11 13 2 9 22 2"/>
            </svg>
            <div v-else class="loading-spinner"></div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chat-view {
  height: 100%;
  display: flex;
  justify-content: center;
  background-color: var(--color-bg-primary);
}

.chat-container {
  width: 100%;
  max-width: 900px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  scroll-behavior: smooth;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  color: var(--color-text-secondary);
}

.empty-icon {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state h2 {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 8px;
}

.empty-state p {
  font-size: 16px;
  max-width: 400px;
}

.input-container {
  padding: 16px 24px 24px;
  background-color: var(--color-bg-primary);
  border-top: 1px solid var(--color-border);
}

.input-wrapper {
  display: flex;
  gap: 12px;
  align-items: flex-end;
  max-width: 100%;
}

.chat-input {
  flex: 1;
  padding: 12px 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background-color: var(--color-bg-secondary);
  color: var(--color-text-primary);
  font-size: 15px;
  line-height: 1.5;
  resize: none;
  max-height: 150px;
  overflow-y: auto;
}

.chat-input:focus {
  outline: none;
  border-color: var(--color-accent);
}

.chat-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.send-button {
  flex-shrink: 0;
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--color-accent);
  color: white;
  border: none;
  border-radius: 8px;
  transition: background-color 0.2s ease;
}

.send-button:hover:not(:disabled) {
  background-color: var(--color-accent-hover);
}

.send-button:disabled {
  opacity: 0.5;
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

@media (max-width: 768px) {
  .messages-container {
    padding: 16px;
  }

  .input-container {
    padding: 12px 16px 16px;
  }

  .empty-state h2 {
    font-size: 20px;
  }

  .empty-state p {
    font-size: 14px;
  }
}
</style>
