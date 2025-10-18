<script setup lang="ts">
import { ref, onMounted } from "vue";
import DocumentUpload from "../components/DocumentUpload.vue";
import ManualInput from "../components/ManualInput.vue";
import TaskStatus from "../components/TaskStatus.vue";
import type { UploadTask, DocumentMetadata } from "../types";
import { apiService } from "../services/api";
import { FolderIcon, PencilIcon } from "lucide-vue-next";

const tasks = ref<UploadTask[]>([]);
const activeTab = ref<"upload" | "manual">("upload");

const handleUpload = async (file: File, metadata: DocumentMetadata) => {
    try {
        const response = await apiService.uploadDocument(file, metadata);
        
        const newTask: UploadTask = {
            id: response.task_id,
            filename: file.name,
            status: "processing",
            uploadedAt: new Date(),
        };

        tasks.value.unshift(newTask);
        
        // Start polling for task status
        pollTaskStatus(response.task_id);
    } catch (error) {
        console.error('Upload error:', error);
        // Add error task to show failure
        const errorTask: UploadTask = {
            id: Date.now().toString(),
            filename: file.name,
            status: "failed",
            uploadedAt: new Date(),
            error: error instanceof Error ? error.message : 'Upload failed',
        };
        tasks.value.unshift(errorTask);
    }
};

const handleManualSave = async (
    content: string,
    metadata: DocumentMetadata,
) => {
    try {
        const title = metadata.title || `manual-input-${new Date().toISOString().split("T")[0]}.md`;
        const response = await apiService.uploadText(content, title, metadata);
        
        const newTask: UploadTask = {
            id: response.task_id,
            filename: title,
            status: "processing",
            uploadedAt: new Date(),
        };

        tasks.value.unshift(newTask);
        
        // Start polling for task status
        pollTaskStatus(response.task_id);
    } catch (error) {
        console.error('Manual save error:', error);
        // Add error task to show failure
        const errorTask: UploadTask = {
            id: Date.now().toString(),
            filename: `manual-input-${new Date().toISOString().split("T")[0]}.md`,
            status: "failed",
            uploadedAt: new Date(),
            error: error instanceof Error ? error.message : 'Save failed',
        };
        tasks.value.unshift(errorTask);
    }
};

const pollTaskStatus = async (taskId: string) => {
    const pollInterval = 2000; // Poll every 2 seconds
    const maxAttempts = 30; // Max 1 minute of polling
    let attempts = 0;

    const poll = async () => {
        try {
            const status = await apiService.getTaskStatus(taskId);
            const task = tasks.value.find((t) => t.id === taskId);
            
            if (task) {
                task.status = status.status;
                if (status.error) {
                    task.error = status.error;
                }
            }

            // Continue polling if still processing and under max attempts
            if (status.status === 'processing' && attempts < maxAttempts) {
                attempts++;
                setTimeout(poll, pollInterval);
            }
        } catch (error) {
            console.error('Error polling task status:', error);
            const task = tasks.value.find((t) => t.id === taskId);
            if (task) {
                task.status = "failed";
                task.error = 'Failed to check status';
            }
        }
    };

    // Start polling
    setTimeout(poll, pollInterval);
};

// Load existing tasks on mount
onMounted(async () => {
    try {
        const response = await apiService.listDocuments();
        // Convert document list to task format for display
        const existingTasks: UploadTask[] = response.documents.map(doc => ({
            id: doc.id,
            filename: doc.title,
            status: doc.status === 'indexed' ? 'succeeded' : 'processing',
            uploadedAt: new Date(doc.created_at),
        }));
        tasks.value = existingTasks;
    } catch (error) {
        console.error('Error loading documents:', error);
    }
});
</script>

<template>
    <div class="h-full overflow-y-auto bg-white dark:bg-gray-950">
        <div class="max-w-[1200px] mx-auto py-8 px-6 md:py-6 md:px-4">
            <div class="mb-10">
                <h1
                    class="text-3xl md:text-2xl font-bold text-gray-900 dark:text-gray-100 mb-2"
                >
                    Wissensverwaltung
                </h1>
                <p
                    class="text-base md:text-sm text-gray-600 dark:text-gray-300"
                >
                    Verwalten Sie Ihr Wissensspeicher durch Upload oder manuelle
                    Eingabe
                </p>
            </div>

            <div class="flex flex-col gap-8">
                <div
                    class="md:flex-col md:gap-0 flex gap-2 border-b border-gray-300 dark:border-gray-700 mb-6"
                >
                    <button
                        @click="activeTab = 'upload'"
                        class="px-5 py-3 bg-transparent border-0 border-b-2 font-medium text-sm cursor-pointer transition-all duration-200 flex items-center gap-2 md:border-b md:rounded-none"
                        :class="
                            activeTab === 'upload'
                                ? 'text-accent dark:text-white border-b-accent dark:border-b-accent-light bg-gray-100 dark:bg-gray-900 md:border-b-gray-300 md:dark:border-b-gray-700 md:border-l-[3px] md:border-l-accent md:dark:border-l-accent-light'
                                : 'text-gray-600 dark:text-gray-300 border-b-transparent hover:text-gray-900 dark:hover:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-800'
                        "
                    >
                        <FolderIcon :size="16" class="mr-1.5" />
                        Dokument Upload
                    </button>
                    <button
                        @click="activeTab = 'manual'"
                        class="px-5 py-3 bg-transparent border-0 border-b-2 font-medium text-sm cursor-pointer transition-all duration-200 flex items-center gap-2 md:border-b md:rounded-none"
                        :class="
                            activeTab === 'manual'
                                ? 'text-accent dark:text-white border-b-accent dark:border-b-accent-light bg-gray-100 dark:bg-gray-900 md:border-b-gray-300 md:dark:border-b-gray-700 md:border-l-[3px] md:border-l-accent md:dark:border-l-accent-light'
                                : 'text-gray-600 dark:text-gray-300 border-b-transparent hover:text-gray-900 dark:hover:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-800'
                        "
                    >
                        <PencilIcon :size="16" class="mr-1.5" />
                        Manuelle Eingabe
                    </button>
                </div>

                <div class="min-h-[400px]">
                    <section
                        v-if="activeTab === 'upload'"
                        class="flex flex-col gap-6"
                    >
                        <h2
                            class="text-xl font-semibold text-gray-900 dark:text-gray-100"
                        >
                            Dokument Upload
                        </h2>
                        <DocumentUpload @upload="handleUpload" />
                    </section>

                    <section
                        v-if="activeTab === 'manual'"
                        class="flex flex-col gap-6"
                    >
                        <ManualInput @save-content="handleManualSave" />
                    </section>
                </div>

                <section class="flex flex-col gap-6">
                    <h2
                        class="text-xl font-semibold text-gray-900 dark:text-gray-100"
                    >
                        Indexierungs-Status
                    </h2>
                    <TaskStatus :tasks="tasks" />
                </section>
            </div>
        </div>
    </div>
</template>
