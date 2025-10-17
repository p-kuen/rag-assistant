<script setup lang="ts">
import { ref } from "vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import MarkdownPreview from "./MarkdownPreview.vue";
import type { DocumentMetadata } from "../types";

const emit = defineEmits<{
    "save-content": [content: string, metadata: DocumentMetadata];
}>();

const markdownContent = ref("");
const tags = ref("");
const documentType = ref("");
const documentDate = ref("");

const handleContentUpdate = (content: string) => {
    markdownContent.value = content;
};

const saveContent = () => {
    if (!markdownContent.value.trim()) {
        alert("Please enter some content before saving.");
        return;
    }

    const metadata: DocumentMetadata = {
        tags: tags.value
            ? tags.value.split(",").map((t) => t.trim())
            : undefined,
        type: documentType.value || undefined,
        date: documentDate.value || undefined,
    };

    emit("save-content", markdownContent.value, metadata);

    // Reset form
    markdownContent.value = "";
    tags.value = "";
    documentType.value = "";
    documentDate.value = "";
};

const clearContent = () => {
    markdownContent.value = "";
    tags.value = "";
    documentType.value = "";
    documentDate.value = "";
};
</script>

<template>
    <div class="flex flex-col gap-6">
        <div class="text-center">
            <h2
                class="text-2xl md:text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2"
            >
                Manuelle Eingabe
            </h2>
            <p class="text-sm text-gray-600 dark:text-gray-300">
                Erstellen Sie Inhalte direkt mit dem Markdown-Editor
            </p>
        </div>

        <div
            class="grid lg:grid-cols-2 grid-cols-1 gap-4 h-[500px] lg:min-h-[400px] min-h-[600px]"
        >
            <div
                class="flex flex-col h-full lg:h-auto lg:min-h-0 min-h-[300px]"
            >
                <MarkdownEditor
                    :content="markdownContent"
                    @update:content="handleContentUpdate"
                    placeholder="Geben Sie hier Ihr Markdown ein..."
                />
            </div>

            <div
                class="flex flex-col h-full lg:h-auto lg:min-h-0 min-h-[300px]"
            >
                <MarkdownPreview :content="markdownContent" />
            </div>
        </div>

        <div class="bg-gray-50 dark:bg-gray-800 rounded-xl p-5">
            <h3
                class="text-base font-semibold text-gray-900 dark:text-gray-100 mb-4"
            >
                Metadaten
            </h3>
            <div class="flex flex-col gap-4">
                <div class="flex flex-col gap-1.5">
                    <label
                        for="manual-tags"
                        class="text-[13px] font-medium text-gray-900 dark:text-gray-100"
                    >
                        Tags (durch Komma getrennt)
                    </label>
                    <input
                        v-model="tags"
                        id="manual-tags"
                        type="text"
                        placeholder="z.B. Architektur, API, Anleitung"
                        class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent placeholder:text-gray-400 dark:placeholder:text-gray-500"
                    />
                </div>
                <div class="grid md:grid-cols-2 grid-cols-1 gap-4">
                    <div class="flex flex-col gap-1.5">
                        <label
                            for="manual-type"
                            class="text-[13px] font-medium text-gray-900 dark:text-gray-100"
                        >
                            Dokumenttyp
                        </label>
                        <input
                            v-model="documentType"
                            id="manual-type"
                            type="text"
                            placeholder="z.B. Anleitung, Referenz"
                            class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent placeholder:text-gray-400 dark:placeholder:text-gray-500"
                        />
                    </div>
                    <div class="flex flex-col gap-1.5">
                        <label
                            for="manual-date"
                            class="text-[13px] font-medium text-gray-900 dark:text-gray-100"
                        >
                            Datum
                        </label>
                        <input
                            v-model="documentDate"
                            id="manual-date"
                            type="date"
                            class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent"
                        />
                    </div>
                </div>
            </div>
        </div>

        <div
            class="flex md:flex-row flex-col md:justify-end justify-stretch gap-3"
        >
            <button
                @click="clearContent"
                class="md:w-auto w-full px-6 py-3 bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-300 border border-gray-300 dark:border-gray-700 rounded-md font-medium text-sm transition-all hover:bg-white hover:dark:bg-gray-900 hover:text-gray-900 hover:dark:text-gray-100"
            >
                Löschen
            </button>
            <button
                @click="saveContent"
                class="md:w-auto w-full px-6 py-3 bg-accent text-white rounded-md font-medium text-sm transition-all hover:bg-accent-hover disabled:bg-gray-100 disabled:dark:bg-gray-800 disabled:text-gray-400 disabled:dark:text-gray-500 disabled:cursor-not-allowed"
                :disabled="!markdownContent.trim()"
            >
                Speichern & Indexieren
            </button>
        </div>
    </div>
</template>
