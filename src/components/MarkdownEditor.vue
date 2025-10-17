<script setup lang="ts">
import { ref, watch } from 'vue'

const emit = defineEmits<{
  'update:content': [content: string]
}>()

const props = defineProps<{
  content: string
  placeholder?: string
}>()

const editorContent = ref(props.content)

watch(() => props.content, (newContent) => {
  editorContent.value = newContent
})

watch(editorContent, (newContent) => {
  emit('update:content', newContent)
})

const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement
  editorContent.value = target.value
}

const insertMarkdown = (syntax: string) => {
  const textarea = document.getElementById('markdown-editor') as HTMLTextAreaElement
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selectedText = editorContent.value.substring(start, end)
  
  let replacement = ''
  
  switch (syntax) {
    case 'bold':
      replacement = `**${selectedText || 'bold text'}**`
      break
    case 'italic':
      replacement = `*${selectedText || 'italic text'}*`
      break
    case 'heading':
      replacement = `# ${selectedText || 'Heading'}`
      break
    case 'link':
      replacement = `[${selectedText || 'link text'}](url)`
      break
    case 'code':
      replacement = `\`${selectedText || 'code'}\``
      break
    case 'codeblock':
      replacement = `\`\`\`\n${selectedText || 'code block'}\n\`\`\``
      break
    case 'list':
      replacement = `- ${selectedText || 'list item'}`
      break
    case 'quote':
      replacement = `> ${selectedText || 'quote'}`
      break
  }
  
  const newContent = editorContent.value.substring(0, start) + replacement + editorContent.value.substring(end)
  editorContent.value = newContent
  
  // Set cursor position after the inserted text
  setTimeout(() => {
    const newCursorPos = start + replacement.length
    textarea.setSelectionRange(newCursorPos, newCursorPos)
    textarea.focus()
  }, 0)
}
</script>

<template>
  <div class="markdown-editor">
    <div class="editor-toolbar">
      <button @click="insertMarkdown('bold')" class="toolbar-btn" title="Bold">
        <strong>B</strong>
      </button>
      <button @click="insertMarkdown('italic')" class="toolbar-btn" title="Italic">
        <em>I</em>
      </button>
      <button @click="insertMarkdown('heading')" class="toolbar-btn" title="Heading">
        H1
      </button>
      <button @click="insertMarkdown('link')" class="toolbar-btn" title="Link">
        🔗
      </button>
      <button @click="insertMarkdown('code')" class="toolbar-btn" title="Inline Code">
        &lt;/&gt;
      </button>
      <button @click="insertMarkdown('codeblock')" class="toolbar-btn" title="Code Block">
        📝
      </button>
      <button @click="insertMarkdown('list')" class="toolbar-btn" title="List">
        • List
      </button>
      <button @click="insertMarkdown('quote')" class="toolbar-btn" title="Quote">
        "
      </button>
    </div>
    
    <textarea
      id="markdown-editor"
      v-model="editorContent"
      @input="handleInput"
      :placeholder="placeholder || 'Enter your markdown content here...'"
      class="editor-textarea"
      spellcheck="false"
    ></textarea>
  </div>
</template>

<style scoped>
.markdown-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  overflow: hidden;
  background-color: var(--color-bg-primary);
}

.editor-toolbar {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
  background-color: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  flex-wrap: wrap;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  height: 28px;
  padding: 4px 8px;
  background: none;
  border: 1px solid transparent;
  border-radius: 4px;
  color: var(--color-text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--color-bg-tertiary);
  border-color: var(--color-border);
  color: var(--color-text-primary);
}

.toolbar-btn:active {
  background-color: var(--color-accent);
  color: white;
  border-color: var(--color-accent);
}

.editor-textarea {
  flex: 1;
  width: 100%;
  padding: 16px;
  border: none;
  outline: none;
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: none;
  tab-size: 2;
}

.editor-textarea::placeholder {
  color: var(--color-text-tertiary);
}

.editor-textarea:focus {
  outline: none;
}

/* Syntax highlighting simulation */
.editor-textarea {
  background-image: 
    linear-gradient(transparent 50%, rgba(0, 0, 0, 0.03) 50%);
  background-size: 100% 2em;
  background-attachment: local;
}

@media (max-width: 768px) {
  .editor-toolbar {
    padding: 6px 8px;
    gap: 2px;
  }
  
  .toolbar-btn {
    min-width: 28px;
    height: 24px;
    font-size: 11px;
  }
  
  .editor-textarea {
    padding: 12px;
    font-size: 13px;
  }
}
</style>

