<script setup lang="ts">
import { ref, nextTick, Ref } from "vue";
import ChatMessage from "../components/ChatMessage.vue";
import type { Message } from "../types";
import { apiService } from "../services/api";
import { LoaderCircleIcon, MessageSquare, Send } from "lucide-vue-next";

const messages = ref<Ref<Message>[]>([]);
const userInput = ref("");
const isLoading = ref(false);
const messagesContainer = ref<HTMLElement | null>(null);
const currentSessionId = ref<string | undefined>(undefined);

const scrollToBottom = () => {
    nextTick(() => {
        if (messagesContainer.value) {
            messagesContainer.value.scrollTop =
                messagesContainer.value.scrollHeight;
        }
    });
};

const sendMessage = async () => {
    const query = userInput.value.trim();
    if (!query || isLoading.value) return;

    const userMessage = ref<Message>({
        id: Date.now().toString(),
        type: "user",
        content: query,
        timestamp: new Date(),
    });

    messages.value.push(userMessage);
    userInput.value = "";
    scrollToBottom();

    const assistantMessage = ref<Message>({
        id: (Date.now() + 1).toString(),
        type: "assistant",
        content: "Folgende Ergebnisse habe ich gefunden:\n",
        sources: [],
        timestamp: new Date(),
    });

    messages.value.push(assistantMessage);
    isLoading.value = true;

    try {
        await apiService.chatStream(
            query,
            (result: string) => {
                console.log("result", result);
                assistantMessage.value.content += "\n\n";
                assistantMessage.value.content += result;
                scrollToBottom();
            },
            () => {
                console.log("complete");
                isLoading.value = false;
                scrollToBottom();
            },
            (error: Error) => {
                console.error("Chat error:", error);
                assistantMessage.value.content =
                    "Entschuldigung, bei der Verarbeitung Ihrer Anfrage ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut.";
                isLoading.value = false;
                scrollToBottom();
            },
            currentSessionId.value,
        );
    } catch (error) {
        console.error("Chat error:", error);
        assistantMessage.value.content =
            "Entschuldigung, bei der Verarbeitung Ihrer Anfrage ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut.";
        isLoading.value = false;
        scrollToBottom();
    }
};

const handleKeyPress = (event: KeyboardEvent) => {
    if (event.key === "Enter" && !event.shiftKey) {
        event.preventDefault();
        sendMessage();
    }
};
</script>

<template>
    <div class="h-full flex justify-center bg-white dark:bg-gray-950">
        <div class="w-full max-w-[900px] h-full flex flex-col">
            <div
                ref="messagesContainer"
                class="flex-1 overflow-y-auto p-6 md:p-4 scroll-smooth"
            >
                <div
                    v-if="messages.length === 0"
                    class="flex flex-col items-center justify-center h-full text-center text-gray-600 dark:text-gray-400"
                >
                    <div class="mb-4 opacity-50">
                        <MessageSquare :size="64" />
                    </div>
                    <h2
                        class="text-2xl md:text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2"
                    >
                        Starten Sie ein Gespräch
                    </h2>
                    <p class="text-base md:text-sm max-w-[400px]">
                        Stellen Sie Fragen zu Ihren Dokumenten und erhalten Sie
                        KI-gestützte Antworten mit Quellenangaben.
                    </p>
                </div>
                <ChatMessage
                    v-for="message in messages"
                    :key="message.value.id"
                    :message="message"
                />
            </div>
            <div
                class="p-4 px-6 pb-6 md:p-3 md:px-4 md:pb-4 bg-white dark:bg-gray-950 border-t border-gray-300 dark:border-gray-700"
            >
                <div class="flex gap-3 items-end max-w-full">
                    <textarea
                        v-model="userInput"
                        @keypress="handleKeyPress"
                        placeholder="Stellen Sie eine Frage zu Ihren Dokumenten..."
                        rows="1"
                        :disabled="isLoading"
                        class="flex-1 px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-lg bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-[15px] leading-6 resize-none max-h-[150px] overflow-y-auto focus:outline-none focus:border-accent disabled:opacity-60 disabled:cursor-not-allowed"
                    />
                    <button
                        @click="sendMessage"
                        :disabled="!userInput.trim() || isLoading"
                        class="flex-shrink-0 w-11 h-11 flex items-center justify-center bg-accent text-white border-0 rounded-lg transition-colors duration-200 hover:bg-accent-hover disabled:opacity-50"
                    >
                        <Send v-if="!isLoading" :size="20" />
                        <LoaderCircleIcon
                            v-else
                            :size="20"
                            class="animate-spin"
                        />
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
