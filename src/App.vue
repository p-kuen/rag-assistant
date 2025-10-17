<script setup lang="ts">
import { useRoute } from "vue-router";
import { computed } from "vue";

const route = useRoute();
const isAdminRoute = computed(() => route.path === "/admin");
</script>

<template>
    <div class="flex flex-col h-screen">
        <nav
            class="bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-700 sticky top-0 z-[100]"
        >
            <div
                class="max-w-[1400px] mx-auto px-6 md:px-4 flex justify-between items-center h-16 md:h-14"
            >
                <div>
                    <h1
                        class="text-xl md:text-lg font-semibold text-gray-900 dark:text-gray-100"
                    >
                        RAG Assistant
                    </h1>
                </div>
                <div class="flex gap-2">
                    <router-link
                        to="/"
                        class="px-4 py-2 md:px-3 md:py-1.5 md:text-sm rounded-md no-underline font-medium transition-all duration-200"
                        :class="
                            !isAdminRoute
                                ? 'bg-accent text-white'
                                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100'
                        "
                    >
                        Chat
                    </router-link>
                    <router-link
                        to="/admin"
                        class="px-4 py-2 md:px-3 md:py-1.5 md:text-sm rounded-md no-underline font-medium transition-all duration-200"
                        :class="
                            isAdminRoute
                                ? 'bg-accent text-white'
                                : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100'
                        "
                    >
                        Wissensverwaltung
                    </router-link>
                </div>
            </div>
        </nav>
        <main class="flex-1 overflow-hidden">
            <router-view />
        </main>
    </div>
</template>
