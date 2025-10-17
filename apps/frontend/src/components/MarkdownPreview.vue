<script setup lang="ts">
import { computed } from "vue";
import VueMarkdownRender from "vue-markdown-render";

const props = defineProps<{
    content: string;
}>();

const hasContent = computed(() => {
    return props.content.trim().length > 0;
});
</script>

<template>
    <div
        class="flex flex-col h-full border border-gray-300 dark:border-gray-700 rounded-lg overflow-hidden bg-white dark:bg-gray-900"
    >
        <div
            class="px-4 py-3 bg-gray-50 dark:bg-gray-800 border-b border-gray-300 dark:border-gray-700"
        >
            <h3
                class="m-0 text-sm font-semibold text-gray-900 dark:text-gray-100"
            >
                Live Preview
            </h3>
        </div>
        <div
            class="flex-1 p-4 overflow-y-auto text-sm leading-relaxed text-gray-900 dark:text-gray-100"
        >
            <div
                v-if="!hasContent"
                class="flex items-center justify-center h-full text-gray-400 dark:text-gray-600 italic"
            >
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
/* Markdown content styling - plain CSS as @apply doesn't work well with Tailwind v4 */
.markdown-content :deep(h1) {
    font-size: 1.5rem;
    font-weight: 700;
    color: rgb(17 24 39);
    margin-top: 1.5rem;
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 2px solid rgb(209 213 219);
}

.markdown-content :deep(h2) {
    font-size: 1.25rem;
    font-weight: 600;
    color: rgb(17 24 39);
    margin-top: 1.25rem;
    margin-bottom: 0.75rem;
    padding-bottom: 0.375rem;
    border-bottom: 1px solid rgb(209 213 219);
}

.markdown-content :deep(h3) {
    font-size: 1.125rem;
    font-weight: 600;
    color: rgb(17 24 39);
    margin-top: 1rem;
    margin-bottom: 0.5rem;
}

.markdown-content :deep(h4) {
    font-size: 1rem;
    font-weight: 600;
    color: rgb(17 24 39);
    margin-top: 0.75rem;
    margin-bottom: 0.375rem;
}

.markdown-content :deep(h5),
.markdown-content :deep(h6) {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgb(17 24 39);
    margin-top: 0.625rem;
    margin-bottom: 0.25rem;
}

.markdown-content :deep(p) {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
    color: rgb(17 24 39);
}

.markdown-content :deep(strong) {
    font-weight: 600;
    color: rgb(17 24 39);
}

.markdown-content :deep(em) {
    font-style: italic;
    color: rgb(17 24 39);
}

.markdown-content :deep(code) {
    background-color: rgb(243 244 246);
    color: #0066cc;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-family: Monaco, Menlo, "Ubuntu Mono", monospace;
    font-size: 13px;
}

.markdown-content :deep(pre) {
    background-color: rgb(243 244 246);
    border: 1px solid rgb(209 213 219);
    border-radius: 0.375rem;
    padding: 1rem;
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
    overflow-x: auto;
}

.markdown-content :deep(pre code) {
    background: transparent;
    color: rgb(17 24 39);
    padding: 0;
    border-radius: 0;
    font-size: 13px;
}

.markdown-content :deep(blockquote) {
    border-left: 4px solid #0066cc;
    background-color: rgb(249 250 251);
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
    padding: 0.75rem 1rem;
    color: rgb(75 85 99);
    font-style: italic;
}

.markdown-content :deep(ul),
.markdown-content :deep(ol) {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
    padding-left: 1.5rem;
}

.markdown-content :deep(li) {
    margin-top: 0.25rem;
    margin-bottom: 0.25rem;
    color: rgb(17 24 39);
}

.markdown-content :deep(a) {
    color: #0066cc;
    text-decoration: none;
    border-bottom: 1px solid transparent;
    transition: border-color 0.2s;
}

.markdown-content :deep(a:hover) {
    border-bottom-color: #0066cc;
}

.markdown-content :deep(table) {
    width: 100%;
    border-collapse: collapse;
    margin-top: 0.75rem;
    margin-bottom: 0.75rem;
    border: 1px solid rgb(209 213 219);
    border-radius: 0.375rem;
    overflow: hidden;
}

.markdown-content :deep(th),
.markdown-content :deep(td) {
    padding: 0.5rem 0.75rem;
    text-align: left;
    border-bottom: 1px solid rgb(209 213 219);
}

.markdown-content :deep(th) {
    background-color: rgb(249 250 251);
    font-weight: 600;
    color: rgb(17 24 39);
}

.markdown-content :deep(td) {
    color: rgb(17 24 39);
}

.markdown-content :deep(hr) {
    border: none;
    height: 1px;
    background-color: rgb(209 213 219);
    margin-top: 1.25rem;
    margin-bottom: 1.25rem;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
    .markdown-content :deep(h1),
    .markdown-content :deep(h2),
    .markdown-content :deep(h3),
    .markdown-content :deep(h4),
    .markdown-content :deep(h5),
    .markdown-content :deep(h6),
    .markdown-content :deep(p),
    .markdown-content :deep(strong),
    .markdown-content :deep(em),
    .markdown-content :deep(li),
    .markdown-content :deep(th),
    .markdown-content :deep(td),
    .markdown-content :deep(pre code) {
        color: rgb(249 250 251);
    }

    .markdown-content :deep(h1) {
        border-bottom-color: rgb(55 65 81);
    }

    .markdown-content :deep(h2) {
        border-bottom-color: rgb(55 65 81);
    }

    .markdown-content :deep(code) {
        background-color: rgb(55 65 81);
        color: #4d9fff;
    }

    .markdown-content :deep(pre) {
        background-color: rgb(55 65 81);
        border-color: rgb(75 85 99);
    }

    .markdown-content :deep(blockquote) {
        background-color: rgb(31 41 55);
        color: rgb(156 163 175);
    }

    .markdown-content :deep(table) {
        border-color: rgb(55 65 81);
    }

    .markdown-content :deep(th),
    .markdown-content :deep(td) {
        border-bottom-color: rgb(55 65 81);
    }

    .markdown-content :deep(th) {
        background-color: rgb(31 41 55);
    }

    .markdown-content :deep(hr) {
        background-color: rgb(55 65 81);
    }
}

@media (max-width: 768px) {
    .markdown-content :deep(h1) {
        font-size: 1.25rem;
    }

    .markdown-content :deep(h2) {
        font-size: 1.125rem;
    }

    .markdown-content :deep(h3) {
        font-size: 1rem;
    }
}
</style>
