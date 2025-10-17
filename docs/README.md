# RAG Assistant - Ressourcenoptimierte Fullstack RAG-Plattform

Eine modulare, Docker-basierte RAG (Retrieval-Augmented Generation) Plattform mit VueJS Frontend und Rust Backend.

## 📁 Monorepo-Struktur

```
rag-assistant/
├── apps/
│   ├── frontend/              # VueJS Chat & Admin Interface
│   └── backend-orchestrator/  # Rust Backend für RAG-Pipeline
├── services/
│   ├── meilisearch/          # Vektor- und Metadaten-Speicher
│   ├── embedding-api/        # Text Embeddings Service (TEI)
│   └── llm-inference/        # LLM Inference Service (llama.cpp)
├── docker/
│   ├── docker-compose.yml    # Orchestrierung aller Services
│   ├── frontend.Dockerfile
│   ├── backend.Dockerfile
│   └── .env.example
├── docs/
│   ├── README.md             # Dieser Dokument
│   ├── ARCHITECTURE.md       # Architektur-Übersicht
│   └── DEPLOYMENT.md         # Deployment-Anleitung
└── pnpm-workspace.yaml       # Pnpm Workspace-Konfiguration
```

## 🚀 Schnellstart

### Voraussetzungen

- **Node.js** >= 18.0.0
- **Pnpm** >= 8.0.0
- **Rust** >= 1.75 (für Backend-Entwicklung)
- **Docker** und **Docker Compose**

### 1. Repository klonen und Dependencies installieren

```bash
# Dependencies für alle Workspaces installieren
pnpm install
```

### 2. Docker Services starten

Zuerst müssen Sie ein LLM-Modell herunterladen:

```bash
# Beispiel: Gemma-2-2B quantisiert (Q4_K_M ~1.5GB)
docker run -v rag-assistant_llm-models:/models alpine sh -c \
  "wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf -O /models/model.gguf"
```

Dann die Umgebungsvariablen konfigurieren:

```bash
# .env Datei erstellen
cp docker/.env.example docker/.env

# .env bearbeiten und API-Keys setzen
# MEILISEARCH_API_KEY=your_secure_key
```

Alle Services starten:

```bash
# Alle Docker Services starten
pnpm docker:up

# Logs verfolgen
pnpm docker:logs
```

### 3. Frontend Development

```bash
# Frontend im Dev-Modus starten
pnpm dev:frontend

# Oder direkt im Frontend-Ordner
cd apps/frontend
pnpm dev
```

Frontend ist erreichbar unter: http://localhost:5173

### 4. Backend Development

```bash
# Backend im Dev-Modus starten (erfordert cargo-watch)
pnpm dev:backend

# Oder direkt im Backend-Ordner
cd apps/backend-orchestrator
cargo run
```

Backend API ist erreichbar unter: http://localhost:8080

## 🏗️ Komponenten-Übersicht

### Frontend (`apps/frontend`)

- **Framework:** VueJS 3 + TypeScript + Vite
- **Styling:** Tailwind CSS v4
- **Routing:** Vue Router
- **Features:**
  - Chat-Assistent Interface (`/chat`)
  - Wissensverwaltung Dashboard (`/admin/knowledge`)
  - Dokument-Upload mit Metadaten
  - Manuelle Markdown-Eingabe
  - Task-Status-Überwachung

### Backend Orchestrator (`apps/backend-orchestrator`)

- **Framework:** Rust (Axum/Actix-web + Tokio)
- **Aufgaben:**
  - RAG-Pipeline Orchestrierung
  - API Gateway für Frontend
  - Koordination zwischen Services
  - Dokument-Chunking und -Processing

**API Endpoints:**
- `GET /health` - Health Check
- `POST /api/chat` - Chat Completions
- `POST /api/documents` - Dokument Upload
- `GET /api/documents` - Dokument Liste

### Meilisearch Service (`services/meilisearch`)

- **Image:** `getmeili/meilisearch:v1.7`
- **Port:** 7700 (intern)
- **Aufgaben:**
  - Vektor- und Keyword-Suche (Hybrid Search)
  - Speicherung von Document Chunks
  - Metadaten-Verwaltung

### Embedding API (`services/embedding-api`)

- **Image:** `ghcr.io/huggingface/text-embeddings-inference:cpu-1.5`
- **Port:** 8080 (intern)
- **Model:** BAAI/bge-small-en-v1.5 (konfigurierbar)
- **API:** OpenAI-kompatibel

### LLM Inference (`services/llm-inference`)

- **Image:** `ghcr.io/ggerganov/llama.cpp:server`
- **Port:** 8080 (intern)
- **Model:** Kompaktes GGUF-Modell (z.B. Gemma-2-2B)
- **API:** OpenAI-kompatibel Chat Completions

## 🔧 Verfügbare Scripts

### Root-Level

```bash
# Development
pnpm dev              # Alle Apps im Dev-Modus
pnpm dev:frontend     # Nur Frontend
pnpm dev:backend      # Nur Backend

# Build
pnpm build            # Alle Apps bauen
pnpm build:frontend   # Nur Frontend bauen
pnpm build:backend    # Nur Backend bauen

# Docker
pnpm docker:up        # Docker Services starten
pnpm docker:down      # Docker Services stoppen
pnpm docker:logs      # Docker Logs anzeigen
```

## 📦 Ressourcen-Limits

Die Docker-Compose-Konfiguration enthält Ressourcen-Limits für die rechenintensiven Services:

| Service | CPU Limit | Memory Limit | CPU Reservation | Memory Reservation |
|---------|-----------|--------------|-----------------|-------------------|
| Backend | 2.0 | 2GB | 0.5 | 512MB |
| Meilisearch | 2.0 | 4GB | 0.5 | 1GB |
| Embedding API | 2.0 | 4GB | 1.0 | 2GB |
| LLM Inference | 4.0 | 8GB | 2.0 | 4GB |

Diese Limits können in `docker/docker-compose.yml` angepasst werden.

## 🌐 Service-Architektur

```
┌─────────────────┐
│   Frontend      │ :5173 (extern)
│   (VueJS)       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Backend       │ :8080 (extern)
│ Orchestrator    │
│   (Rust)        │
└────────┬────────┘
         │
    ┌────┼────────┬─────────────┐
    ▼    ▼        ▼             ▼
┌────────┐  ┌─────────┐  ┌──────────┐
│Meili-  │  │Embedding│  │   LLM    │
│search  │  │   API   │  │Inference │
└────────┘  └─────────┘  └──────────┘
  :7700       :8080         :8080
 (intern)    (intern)      (intern)
```

Alle internen Services kommunizieren über das `rag-net` Docker-Netzwerk.

## 🔐 Sicherheit

- Interne Services (Meilisearch, Embedding API, LLM) sind **nicht** extern erreichbar
- Nur Frontend und Backend-Orchestrator exponieren externe Ports
- Meilisearch erfordert einen Master Key (in `.env` konfigurieren)
- Docker Container laufen mit non-root User (wo möglich)

## 📚 Weitere Dokumentation

- [Architektur-Übersicht](./ARCHITECTURE.md)
- [Deployment-Anleitung](./DEPLOYMENT.md)
- [API-Dokumentation](./API.md)

## 🤝 Beitragen

Da dies ein Monorepo ist, beachten Sie bitte:

1. Alle Packages verwenden Pnpm Workspaces
2. Shared Dependencies sollten im Root `package.json` deklariert werden
3. Service-spezifische Dependencies gehören in die jeweiligen `package.json`
4. Commit-Messages sollten den Workspace-Prefix enthalten (z.B. `[frontend]`, `[backend]`)

## 📝 Lizenz

[Ihre Lizenz hier]
