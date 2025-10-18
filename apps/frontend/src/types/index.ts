export interface Message {
  id: string;
  type: "user" | "assistant";
  content: string;
  sources?: Source[];
  timestamp: Date;
}

export interface Source {
  title: string;
  filename: string;
  hierarchyPath: string;
  score?: number;
}

export interface UploadTask {
  id: string;
  filename: string;
  status: "pending" | "processing" | "succeeded" | "failed";
  uploadedAt: Date;
  error?: string;
}

export interface DocumentMetadata {
  title?: string;
  tags?: string[];
  type?: string;
  date?: string;
}

export interface DocumentListResponse {
  documents: DocumentInfo[];
}

export interface DocumentInfo {
  id: string;
  title: string;
  status: string;
  created_at: string;
  chunk_count: number;
}
