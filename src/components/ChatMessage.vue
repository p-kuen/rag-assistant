<script setup lang="ts">
import { ref } from "vue";
import type { Message } from "../types";
import SourceAttribution from "./SourceAttribution.vue";

defineProps<{
    message: Message;
}>();

const showSources = ref(false);
</script>

<template>
    <div
        class="flex mb-4 animate-[slideIn_0.3s_ease]"
        :class="message.type === 'user' ? 'justify-end' : ''"
    >
        <div
            class="max-w-[70%] md:max-w-[85%] px-4 py-3 rounded-xl shadow-sm"
            :class="
                message.type === 'user'
                    ? 'bg-accent text-white'
                    : 'bg-gray-100 dark:bg-gray-900'
            "
        >
            <div class="leading-relaxed whitespace-pre-wrap break-words">
                {{ message.content }}
            </div>
            <div
                v-if="message.sources && message.sources.length > 0"
                class="mt-3 pt-3 border-t"
                :class="
                    message.type === 'user'
                        ? 'border-white/20'
                        : 'border-gray-300 dark:border-gray-700'
                "
            >
                <button
                    @click="showSources = !showSources"
                    class="bg-transparent border-0 text-sm font-medium py-1 px-0 underline transition-opacity duration-200 hover:opacity-80"
                    :class="
                        message.type === 'user' ? 'text-white' : 'text-accent'
                    "
                >
                    {{
                        showSources
                            ? "Hide Sources"
                            : `Show Sources (${message.sources.length})`
                    }}
                </button>
                <SourceAttribution
                    v-if="showSources"
                    :sources="message.sources"
                />
            </div>
        </div>
    </div>
</template>

<style scoped>
@keyframes slideIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
</style>
