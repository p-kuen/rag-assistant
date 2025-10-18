<script setup lang="ts">
import { Ref, ref } from "vue";
import type { Message } from "../types";
import SourceAttribution from "./SourceAttribution.vue";
import { watchEffect } from "vue";

const props = defineProps<{
    message: Ref<Message>;
}>();

const showSources = ref(false);

watchEffect(() => {
    console.log("message", props.message);
});
</script>

<template>
    <div
        class="flex mb-4 animate-[slideIn_0.3s_ease]"
        :class="message.value.type === 'user' ? 'justify-end' : ''"
    >
        <div
            class="max-w-[70%] md:max-w-[85%] px-4 py-3 rounded-xl shadow-sm"
            :class="
                message.value.type === 'user'
                    ? 'bg-accent text-gray-900'
                    : 'bg-gray-100 dark:bg-gray-900'
            "
        >
            <div class="leading-relaxed whitespace-pre-wrap break-words">
                {{ message.value.content }}
            </div>
            <div
                v-if="message.value.sources && message.value.sources.length > 0"
                class="mt-3 pt-3 border-t"
                :class="
                    message.value.type === 'user'
                        ? 'border-white/20'
                        : 'border-gray-300 dark:border-gray-700'
                "
            >
                <button
                    @click="showSources = !showSources"
                    class="bg-transparent border-0 text-sm font-medium py-1 px-0 underline transition-opacity duration-200 hover:opacity-80"
                    :class="
                        message.value.type === 'user'
                            ? 'text-white'
                            : 'text-accent'
                    "
                >
                    {{
                        showSources
                            ? "Hide Sources"
                            : `Show Sources (${message.value.sources.length})`
                    }}
                </button>
                <SourceAttribution
                    v-if="showSources"
                    :sources="message.value.sources"
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
