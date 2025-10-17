<script setup lang="ts">
import { computed } from "vue";
import type { UploadTask } from "../types";

const props = defineProps<{
    tasks: UploadTask[];
}>();

const statistics = computed(() => {
    const total = props.tasks.length;
    const processing = props.tasks.filter(
        (t) => t.status === "processing",
    ).length;
    const succeeded = props.tasks.filter(
        (t) => t.status === "succeeded",
    ).length;
    const failed = props.tasks.filter((t) => t.status === "failed").length;

    return {
        total,
        processing,
        succeeded,
        failed,
        successRate: total > 0 ? Math.round((succeeded / total) * 100) : 0,
    };
});
</script>

<template>
    <div class="flex flex-col gap-4">
        <div class="flex justify-end items-center">
            <div
                class="px-3 py-1 bg-gray-100 dark:bg-gray-800 rounded-xl text-sm font-medium text-gray-700 dark:text-gray-300"
            >
                {{ statistics.total }} insgesamt
            </div>
        </div>

        <div
            v-if="statistics.total > 0"
            class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6"
        >
            <div
                class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-center transition-all hover:bg-gray-100 hover:dark:bg-gray-700 hover:border-accent"
            >
                <div class="text-2xl font-bold text-accent mb-1">
                    {{ statistics.succeeded }}
                </div>
                <div
                    class="text-xs font-medium text-gray-700 dark:text-gray-300 uppercase tracking-wider"
                >
                    Erfolgreich
                </div>
            </div>
            <div
                class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-center transition-all hover:bg-gray-100 hover:dark:bg-gray-700 hover:border-accent"
            >
                <div class="text-2xl font-bold text-accent mb-1">
                    {{ statistics.processing }}
                </div>
                <div
                    class="text-xs font-medium text-gray-700 dark:text-gray-300 uppercase tracking-wider"
                >
                    In Bearbeitung
                </div>
            </div>
            <div
                class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-center transition-all hover:bg-gray-100 hover:dark:bg-gray-700 hover:border-accent"
            >
                <div class="text-2xl font-bold text-accent mb-1">
                    {{ statistics.failed }}
                </div>
                <div
                    class="text-xs font-medium text-gray-700 dark:text-gray-300 uppercase tracking-wider"
                >
                    Fehlgeschlagen
                </div>
            </div>
            <div
                class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-center transition-all hover:bg-gray-100 hover:dark:bg-gray-700 hover:border-accent"
            >
                <div class="text-2xl font-bold text-accent mb-1">
                    {{ statistics.successRate }}%
                </div>
                <div
                    class="text-xs font-medium text-gray-700 dark:text-gray-300 uppercase tracking-wider"
                >
                    Erfolgsrate
                </div>
            </div>
        </div>

        <div
            v-if="tasks.length === 0"
            class="flex flex-col items-center justify-center py-16 px-6 text-gray-600 dark:text-gray-300 text-center"
        >
            <svg
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                class="mb-4 opacity-50"
            >
                <polyline points="9 11 12 14 22 4" />
                <path
                    d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"
                />
            </svg>
            <p class="text-base">Noch keine Indexierungsaufgaben</p>
        </div>

        <div
            v-else
            class="bg-gray-50 dark:bg-gray-800 rounded-xl overflow-hidden shadow-sm"
        >
            <div
                class="hidden md:grid grid-cols-[150px_1fr_200px] px-6 py-4 bg-gray-100 dark:bg-gray-700 text-sm font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider"
            >
                <div>Status</div>
                <div>Dateiname</div>
                <div>Hochgeladen</div>
            </div>
            <div class="flex flex-col">
                <div
                    v-for="task in tasks"
                    :key="task.id"
                    class="grid md:grid-cols-[150px_1fr_200px] grid-cols-1 md:gap-0 gap-3 px-6 md:py-4 py-4 border-b border-gray-200 dark:border-gray-700 last:border-b-0 transition-colors hover:bg-gray-100 hover:dark:bg-gray-700"
                >
                    <div class="flex items-center">
                        <span
                            class="inline-flex items-center gap-1.5 px-3 py-1 rounded-xl text-xs font-medium capitalize"
                            :class="{
                                'bg-yellow-100 dark:bg-yellow-900/20 text-yellow-700 dark:text-yellow-400':
                                    task.status === 'processing',
                                'bg-green-100 dark:bg-green-900/20 text-green-700 dark:text-green-400':
                                    task.status === 'succeeded',
                                'bg-red-100 dark:bg-red-900/20 text-red-700 dark:text-red-400':
                                    task.status === 'failed',
                            }"
                        >
                            <span
                                class="flex items-center"
                                v-if="task.status === 'processing'"
                            >
                                <svg
                                    width="14"
                                    height="14"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    class="animate-spin"
                                >
                                    <circle cx="12" cy="12" r="10" />
                                </svg>
                            </span>
                            <span
                                class="flex items-center"
                                v-else-if="task.status === 'succeeded'"
                            >
                                <svg
                                    width="14"
                                    height="14"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <polyline points="20 6 9 17 4 12" />
                                </svg>
                            </span>
                            <span class="flex items-center" v-else>
                                <svg
                                    width="14"
                                    height="14"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <line x1="18" y1="6" x2="6" y2="18" />
                                    <line x1="6" y1="6" x2="18" y2="18" />
                                </svg>
                            </span>
                            {{
                                task.status === "processing"
                                    ? "Verarbeitung"
                                    : task.status === "succeeded"
                                      ? "Erfolgreich"
                                      : "Fehlgeschlagen"
                            }}
                        </span>
                    </div>
                    <div class="flex flex-col gap-1">
                        <span
                            class="font-medium text-gray-900 dark:text-gray-100 overflow-hidden text-ellipsis whitespace-nowrap"
                        >
                            {{ task.filename }}
                        </span>
                        <span
                            v-if="task.error"
                            class="text-xs text-red-600 dark:text-red-400"
                        >
                            {{ task.error }}
                        </span>
                    </div>
                    <div
                        class="flex items-center text-sm text-gray-700 dark:text-gray-300 md:text-base md:before:content-[''] before:content-['Hochgeladen:_'] before:font-medium"
                    >
                        {{ new Date(task.uploadedAt).toLocaleString() }}
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
