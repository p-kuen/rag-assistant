<script setup lang="ts">
import { ref, nextTick } from "vue";
import ChatMessage from "../components/ChatMessage.vue";
import type { Message } from "../types";

const messages = ref<Message[]>([]);
const userInput = ref("");
const isLoading = ref(false);
const messagesContainer = ref<HTMLElement | null>(null);

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

    const userMessage: Message = {
        id: Date.now().toString(),
        type: "user",
        content: query,
        timestamp: new Date(),
    };

    messages.value.push(userMessage);
    userInput.value = "";
    scrollToBottom();

    const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        type: "assistant",
        content: "",
        sources: [],
        timestamp: new Date(),
    };

    messages.value.push(assistantMessage);
    isLoading.value = true;

    try {
        await simulateStreamingResponse(assistantMessage, query);
    } catch (error) {
        assistantMessage.content =
            "Sorry, an error occurred while processing your request.";
    } finally {
        isLoading.value = false;
        scrollToBottom();
    }
};

const simulateStreamingResponse = async (message: Message, query: string) => {
    const sampleResponse = `Based on your query about "${query}", here's what I found in the knowledge base. This response demonstrates the streaming capability where tokens appear one by one in real-time. The system uses RAG (Retrieval-Augmented Generation) to provide accurate, context-aware answers based on your documents.`;

    const sampleSources = [
        {
            title: "Introduction to RAG Systems",
            filename: "rag-overview.md",
            hierarchyPath: "documentation/architecture",
            score: 0.95,
        },
        {
            title: "Implementation Guide",
            filename: "implementation.md",
            hierarchyPath: "documentation/guides",
            score: 0.87,
        },
    ];

    const words = sampleResponse.split(" ");
    for (const word of words) {
        message.content += (message.content ? " " : "") + word;
        scrollToBottom();
        await new Promise((resolve) => setTimeout(resolve, 50));
    }

    message.sources = sampleSources;
    scrollToBottom();
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
                        <svg
                            width="64"
                            height="64"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.5"
                        >
                            <path
                                d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"
                            />
                        </svg>
                    </div>
                    <h2
                        class="text-2xl md:text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2"
                    >
                        Start a conversation
                    </h2>
                    <p class="text-base md:text-sm max-w-[400px]">
                        Ask questions about your documents and get AI-powered
                        answers with source attribution.
                    </p>
                </div>
                <ChatMessage
                    v-for="message in messages"
                    :key="message.id"
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
                        placeholder="Ask a question about your documents..."
                        rows="1"
                        :disabled="isLoading"
                        class="flex-1 px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-lg bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 text-[15px] leading-6 resize-none max-h-[150px] overflow-y-auto focus:outline-none focus:border-accent disabled:opacity-60 disabled:cursor-not-allowed"
                    />
                    <button
                        @click="sendMessage"
                        :disabled="!userInput.trim() || isLoading"
                        class="flex-shrink-0 w-11 h-11 flex items-center justify-center bg-accent text-white border-0 rounded-lg transition-colors duration-200 hover:bg-accent-hover disabled:opacity-50"
                    >
                        <svg
                            v-if="!isLoading"
                            width="20"
                            height="20"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <line x1="22" y1="2" x2="11" y2="13" />
                            <polygon points="22 2 15 22 11 13 2 9 22 2" />
                        </svg>
                        <div
                            v-else
                            class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"
                        ></div>
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>
