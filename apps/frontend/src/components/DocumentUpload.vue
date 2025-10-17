<script setup lang="ts">
import { ref } from "vue";
import type { DocumentMetadata } from "../types";

const emit = defineEmits<{
    upload: [file: File, metadata: DocumentMetadata];
}>();

const isDragging = ref(false);
const selectedFile = ref<File | null>(null);
const tags = ref("");
const documentType = ref("");
const documentDate = ref("");

const handleDragOver = (event: DragEvent) => {
    event.preventDefault();
    isDragging.value = true;
};

const handleDragLeave = () => {
    isDragging.value = false;
};

const handleDrop = (event: DragEvent) => {
    event.preventDefault();
    isDragging.value = false;

    const files = event.dataTransfer?.files;
    if (files && files.length > 0) {
        handleFileSelect(files[0]);
    }
};

const handleFileInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
        handleFileSelect(target.files[0]);
    }
};

const handleFileSelect = (file: File) => {
    if (file.name.endsWith(".md") || file.name.endsWith(".pdf")) {
        selectedFile.value = file;
    } else {
        alert("Please select a .md or .pdf file");
    }
};

const uploadDocument = () => {
    if (!selectedFile.value) return;

    const metadata: DocumentMetadata = {
        tags: tags.value
            ? tags.value.split(",").map((t) => t.trim())
            : undefined,
        type: documentType.value || undefined,
        date: documentDate.value || undefined,
    };

    emit("upload", selectedFile.value, metadata);

    selectedFile.value = null;
    tags.value = "";
    documentType.value = "";
    documentDate.value = "";
};
</script>

<template>
    <div class="flex flex-col gap-6">
        <div
            class="border-2 border-dashed rounded-xl text-center transition-all bg-gray-50 dark:bg-gray-800"
            :class="{
                'border-gray-300 dark:border-gray-700 py-12 px-6':
                    !selectedFile && !isDragging,
                'border-accent bg-gray-100 dark:bg-gray-700 scale-[1.02] py-12 px-6':
                    isDragging,
                'border-gray-400 dark:border-gray-600 border-solid py-6 px-6':
                    selectedFile,
            }"
            @dragover="handleDragOver"
            @dragleave="handleDragLeave"
            @drop="handleDrop"
        >
            <input
                type="file"
                id="file-input"
                accept=".md,.pdf"
                @change="handleFileInput"
                class="hidden"
            />

            <div
                v-if="!selectedFile"
                class="flex flex-col items-center gap-3 text-gray-600 dark:text-gray-300"
            >
                <svg
                    width="48"
                    height="48"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    class="opacity-50"
                >
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                    <polyline points="17 8 12 3 7 8" />
                    <line x1="12" y1="3" x2="12" y2="15" />
                </svg>
                <p
                    class="text-base font-medium text-gray-900 dark:text-gray-100"
                >
                    Drag and drop your document here
                </p>
                <p class="text-sm">or</p>
                <label
                    for="file-input"
                    class="inline-block px-6 py-2 bg-accent text-white rounded-md font-medium cursor-pointer transition-colors hover:bg-accent-hover"
                >
                    Browse files
                </label>
                <p class="text-xs mt-2">Supported: .md, .pdf</p>
            </div>

            <div
                v-else
                class="flex items-center gap-4 p-4 bg-white dark:bg-gray-900 rounded-lg"
            >
                <svg
                    width="32"
                    height="32"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    class="flex-shrink-0 text-accent"
                >
                    <path
                        d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"
                    />
                    <polyline points="13 2 13 9 20 9" />
                </svg>
                <div class="flex-1 text-left min-w-0">
                    <p
                        class="font-medium text-gray-900 dark:text-gray-100 overflow-hidden text-ellipsis whitespace-nowrap"
                    >
                        {{ selectedFile.name }}
                    </p>
                    <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
                        {{ (selectedFile.size / 1024).toFixed(2) }} KB
                    </p>
                </div>
                <button
                    @click="selectedFile = null"
                    class="flex-shrink-0 w-8 h-8 flex items-center justify-center text-gray-600 dark:text-gray-300 rounded-md transition-all hover:bg-gray-100 hover:dark:bg-gray-800 hover:text-red-600 hover:dark:text-red-400"
                >
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                    >
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                </button>
            </div>
        </div>

        <div
            v-if="selectedFile"
            class="flex flex-col gap-4 p-6 bg-gray-50 dark:bg-gray-800 rounded-xl"
        >
            <div class="flex flex-col gap-2">
                <label
                    for="tags"
                    class="text-sm font-medium text-gray-900 dark:text-gray-100"
                >
                    Tags (comma-separated)
                </label>
                <input
                    v-model="tags"
                    id="tags"
                    type="text"
                    placeholder="e.g., architecture, api, guide"
                    class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent placeholder:text-gray-400 dark:placeholder:text-gray-500"
                />
            </div>
            <div class="grid md:grid-cols-2 grid-cols-1 gap-4">
                <div class="flex flex-col gap-2">
                    <label
                        for="type"
                        class="text-sm font-medium text-gray-900 dark:text-gray-100"
                    >
                        Document Type
                    </label>
                    <input
                        v-model="documentType"
                        id="type"
                        type="text"
                        placeholder="e.g., Guide, Reference"
                        class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent placeholder:text-gray-400 dark:placeholder:text-gray-500"
                    />
                </div>
                <div class="flex flex-col gap-2">
                    <label
                        for="date"
                        class="text-sm font-medium text-gray-900 dark:text-gray-100"
                    >
                        Date
                    </label>
                    <input
                        v-model="documentDate"
                        id="date"
                        type="date"
                        class="px-3 py-2.5 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-sm transition-colors focus:outline-none focus:border-accent"
                    />
                </div>
            </div>
            <button
                @click="uploadDocument"
                class="px-6 py-3 bg-accent text-white rounded-md font-medium text-[15px] transition-colors hover:bg-accent-hover"
            >
                Upload and Index
            </button>
        </div>
    </div>
</template>
