<script setup lang="ts">
import type { Source } from "../types";

defineProps<{
    sources: Source[];
}>();
</script>

<template>
    <div class="mt-2">
        <div
            class="flex items-center gap-1.5 text-[13px] font-semibold text-gray-600 dark:text-gray-400 mb-2"
        >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                <path
                    d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0zM5.78 8.75a.75.75 0 0 0 0 1.5h4.44a.75.75 0 0 0 0-1.5H5.78zM6.25 5.75a.75.75 0 0 0 0 1.5h3.5a.75.75 0 0 0 0-1.5h-3.5z"
                />
            </svg>
            <span>Sources</span>
        </div>
        <div class="flex flex-col gap-2">
            <div
                v-for="(source, index) in sources"
                :key="index"
                class="flex gap-3 p-2 bg-gray-100 dark:bg-gray-700 rounded-md text-[13px] transition-transform hover:translate-x-1"
            >
                <div
                    class="flex-shrink-0 w-6 h-6 flex items-center justify-center bg-accent text-white rounded-full font-semibold text-xs"
                >
                    {{ index + 1 }}
                </div>
                <div class="flex-1 min-w-0">
                    <div
                        class="font-medium text-gray-900 dark:text-gray-100 mb-0.5 overflow-hidden text-ellipsis whitespace-nowrap"
                    >
                        {{ source.title }}
                    </div>
                    <div
                        class="text-gray-600 dark:text-gray-400 text-xs overflow-hidden text-ellipsis whitespace-nowrap"
                    >
                        <span class="font-medium">{{ source.filename }}</span>
                        <span v-if="source.hierarchyPath" class="opacity-80">
                            / {{ source.hierarchyPath }}</span
                        >
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
