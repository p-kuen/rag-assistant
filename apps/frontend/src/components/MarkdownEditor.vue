<script setup lang="ts">
import { ref, watch } from "vue";
import { Link as LinkIcon, FileCode as CodeBlockIcon } from "lucide-vue-next";

type IconMap = {
    link: typeof LinkIcon;
    codeblock: typeof CodeBlockIcon;
};

const iconMap = {
    link: LinkIcon,
    codeblock: CodeBlockIcon,
} as const;

const iconFor = (name: keyof IconMap) => {
    // Safe access; will return undefined if not found
    return iconMap[name] ?? null;
};

const emit = defineEmits<{
    "update:content": [content: string];
}>();

const props = defineProps<{
    content: string;
    placeholder?: string;
}>();

const editorContent = ref(props.content);

watch(
    () => props.content,
    (newContent) => {
        editorContent.value = newContent;
    },
);

watch(editorContent, (newContent) => {
    emit("update:content", newContent);
});

const handleInput = (event: Event) => {
    const target = event.target as HTMLTextAreaElement;
    editorContent.value = target.value;
};

const insertMarkdown = (syntax: string) => {
    const textarea = document.getElementById(
        "markdown-editor",
    ) as HTMLTextAreaElement;
    if (!textarea) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const selectedText = editorContent.value.substring(start, end);

    let replacement = "";

    switch (syntax) {
        case "bold":
            replacement = `**${selectedText || "bold text"}**`;
            break;
        case "italic":
            replacement = `*${selectedText || "italic text"}*`;
            break;
        case "heading":
            replacement = `# ${selectedText || "Heading"}`;
            break;
        case "link":
            replacement = `[${selectedText || "link text"}](url)`;
            break;
        case "code":
            replacement = `\`${selectedText || "code"}\``;
            break;
        case "codeblock":
            replacement = "```\\n" + (selectedText || "code block") + "\\n```";
            break;
        case "list":
            replacement = `- ${selectedText || "list item"}`;
            break;
        case "quote":
            replacement = `> ${selectedText || "quote"}`;
            break;
    }

    const newContent =
        editorContent.value.substring(0, start) +
        replacement +
        editorContent.value.substring(end);
    editorContent.value = newContent;

    // Set cursor position after the inserted text
    setTimeout(() => {
        const newCursorPos = start + replacement.length;
        textarea.setSelectionRange(newCursorPos, newCursorPos);
        textarea.focus();
    }, 0);
};
</script>

<template>
    <div
        class="flex flex-col h-full border border-gray-300 dark:border-gray-700 rounded-lg overflow-hidden bg-white dark:bg-gray-950"
    >
        <div
            class="flex gap-1 p-2 px-3 md:p-1.5 md:px-2 bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-700 flex-wrap"
        >
            <button
                @click="insertMarkdown('bold')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Bold"
            >
                <strong>B</strong>
            </button>
            <button
                @click="insertMarkdown('italic')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Italic"
            >
                <em>I</em>
            </button>
            <button
                @click="insertMarkdown('heading')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Heading"
            >
                H1
            </button>
            <button
                @click="insertMarkdown('link')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Link"
            >
                <component :is="iconFor('link')" :size="16" />
            </button>
            <button
                @click="insertMarkdown('code')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Inline Code"
            >
                &lt;/&gt;
            </button>
            <button
                @click="insertMarkdown('codeblock')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Code Block"
            >
                <component :is="iconFor('codeblock')" :size="16" />
            </button>
            <button
                @click="insertMarkdown('list')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="List"
            >
                • List
            </button>
            <button
                @click="insertMarkdown('quote')"
                class="flex items-center justify-center min-w-8 h-7 md:min-w-7 md:h-6 px-2 bg-transparent border border-transparent rounded text-gray-600 dark:text-gray-400 text-xs md:text-[11px] font-medium cursor-pointer transition-all duration-200 hover:bg-gray-200 dark:hover:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-700 hover:text-gray-900 dark:hover:text-gray-100 active:bg-accent active:text-white active:border-accent"
                title="Quote"
            >
                "
            </button>
        </div>

        <textarea
            id="markdown-editor"
            v-model="editorContent"
            @input="handleInput"
            :placeholder="placeholder || 'Enter your markdown content here...'"
            class="flex-1 w-full p-4 md:p-3 border-0 outline-none bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 font-mono text-sm md:text-[13px] leading-relaxed resize-none [tab-size:2] placeholder:text-gray-400 dark:placeholder:text-gray-600 focus:outline-none"
            spellcheck="false"
        ></textarea>
    </div>
</template>
