import type { Source, DocumentMetadata, DocumentListResponse } from "../types";

const API_BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:8080";

export interface ChatRequest {
  message: string;
  session_id?: string;
}

export interface ChatResponse {
  response: string;
  sources: Source[];
  session_id?: string;
}

export interface UploadResponse {
  task_id: string;
  status: string;
}

export interface TaskStatus {
  id: string;
  status: "pending" | "processing" | "succeeded" | "failed";
  progress?: number;
  error?: string;
  created_at: string;
  updated_at: string;
}

class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public response?: any,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const errorText = await response.text();
    throw new ApiError(
      `HTTP ${response.status}: ${errorText}`,
      response.status,
      errorText,
    );
  }

  if (response.headers.get("content-type")?.includes("application/json")) {
    return response.json();
  }

  return response.text() as unknown as T;
}

export class ApiService {
  private baseUrl: string;

  constructor(baseUrl: string = API_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  async uploadDocument(
    file: File,
    metadata?: DocumentMetadata,
  ): Promise<UploadResponse> {
    const formData = new FormData();
    formData.append("file", file);

    if (metadata) {
      formData.append("metadata", JSON.stringify(metadata));
    }

    const response = await fetch(`${this.baseUrl}/api/documents`, {
      method: "POST",
      body: formData,
    });

    return handleResponse<UploadResponse>(response);
  }

  async uploadText(
    content: string,
    title: string,
    metadata?: DocumentMetadata,
  ): Promise<UploadResponse> {
    const formData = new FormData();
    formData.append("content", content);
    formData.append("title", title);

    if (metadata) {
      formData.append("metadata", JSON.stringify(metadata));
    }

    const response = await fetch(`${this.baseUrl}/api/documents`, {
      method: "POST",
      body: formData,
    });

    return handleResponse<UploadResponse>(response);
  }

  async getTaskStatus(taskId: string): Promise<TaskStatus> {
    const response = await fetch(
      `${this.baseUrl}/api/documents/tasks/${taskId}`,
    );
    return handleResponse<TaskStatus>(response);
  }

  async listDocuments(): Promise<DocumentListResponse> {
    const response = await fetch(`${this.baseUrl}/api/documents`);
    return handleResponse<DocumentListResponse>(response);
  }

  async chatStream(
    message: string,
    onResult: (result: string) => void,
    onComplete: () => void,
    onError: (error: Error) => void,
    sessionId?: string,
  ): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/api/chat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          message,
          session_id: sessionId,
        }),
      });

      if (!response.ok) {
        throw new ApiError(`HTTP ${response.status}`, response.status);
      }

      const reader = response.body?.getReader();
      if (!reader) {
        throw new Error("No response body");
      }

      const decoder = new TextDecoder();
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();

        if (done) {
          onComplete();
          break;
        }

        buffer += decoder.decode(value, { stream: true });
        const lines = buffer.split("\n");
        buffer = lines.pop() || "";

        for (const line of lines) {
          if (line.startsWith("data: ")) {
            const data = line.slice(6);

            if (data === "[DONE]") {
              onComplete();
              return;
            }

            console.log("data", data);

            try {
              // Try to parse as JSON (for sources)
              const parsed = JSON.parse(data);
              if ("content" in parsed) {
                onResult(parsed.content);
              } else {
                onResult(data);
              }
            } catch {
              // If not JSON, treat as text chunk
              if (data.trim()) {
                onResult(data);
              }
            }
          }
        }
      }
    } catch (error) {
      onError(error instanceof Error ? error : new Error(String(error)));
    }
  }

  async healthCheck(): Promise<string> {
    const response = await fetch(`${this.baseUrl}/health`);
    return handleResponse<string>(response);
  }
}

// Export singleton instance
export const apiService = new ApiService();
