# Projekt-Struktur

Detaillierte Übersicht der Monorepo-Struktur.

## Root-Level

```
rag-assistant/
├── .bolt/                    # Bolt.new Konfiguration
├── .git/                     # Git Repository
├── .vscode/                  # VS Code Settings
├── apps/                     # Hauptanwendungen
├── services/                 # Microservice-Definitionen
├── docker/                   # Docker-Konfigurationen
├── docs/                     # Dokumentation
├── .env                      # Environment (nicht committen)
├── .gitignore               # Git Ignore-Rules
├── .npmrc                   # NPM Konfiguration
├── Makefile                 # Convenience-Kommandos
├── package.json             # Root Package Definition
├── pnpm-workspace.yaml      # Pnpm Workspace Config
├── QUICKSTART.md            # Schnellstart-Anleitung
└── README.md                # Haupt-Readme
```

## Apps (`apps/`)

### Frontend (`apps/frontend/`)

```
apps/frontend/
├── public/                  # Statische Assets
├── src/
│   ├── assets/             # Bilder, Fonts, etc.
│   ├── components/         # Vue Komponenten
│   │   ├── AppLayout.vue         # Haupt-Layout mit Navigation
│   │   ├── ChatMessage.vue       # Chat-Nachricht Komponente
│   │   ├── DocumentUpload.vue    # Dokument-Upload UI
│   │   ├── ManualInput.vue       # Manuelle Eingabe UI
│   │   ├── MarkdownEditor.vue    # Markdown Editor
│   │   ├── MarkdownPreview.vue   # Markdown Vorschau
│   │   ├── SourceAttribution.vue # Quellen-Anzeige
│   │   └── TaskStatus.vue        # Status-Tabelle
│   ├── router/             # Vue Router Config
│   │   └── index.ts              # Routing-Definition
│   ├── views/              # Page-Level Komponenten
│   │   ├── ChatView.vue          # Chat-Assistent Seite
│   │   └── AdminView.vue         # Wissensverwaltung Seite
│   ├── types/              # TypeScript Types
│   │   └── index.ts
│   ├── App.vue             # Root Vue Komponente
│   └── main.ts             # Entry Point
├── index.html              # HTML Template
├── vite.config.ts          # Vite Konfiguration
├── tsconfig.json           # TypeScript Config
├── tsconfig.app.json       # App-spezifische TS Config
├── tsconfig.node.json      # Node-spezifische TS Config
├── package.json            # Frontend Dependencies
└── pnpm-lock.yaml          # Lock File

**Technologien:**
- Vue 3 (Composition API)
- TypeScript
- Vite
- Tailwind CSS v4
- Vue Router
- Lucide Icons
```

### Backend Orchestrator (`apps/backend-orchestrator/`)

```
apps/backend-orchestrator/
├── src/
│   └── main.rs             # Rust Hauptdatei
├── target/                 # Rust Build-Output (ignoriert)
├── Cargo.toml              # Rust Dependencies
├── package.json            # NPM Scripts (für Monorepo)
└── .env.example            # Environment Template

**Technologien:**
- Rust (Edition 2021)
- Axum (Web Framework)
- Tokio (Async Runtime)
- Reqwest (HTTP Client)
- Serde (Serialization)

**Hauptfunktionen:**
- API Gateway für Frontend
- RAG-Pipeline Orchestrierung
- Service-zu-Service Kommunikation
- Dokument-Chunking
- Embedding-Koordination
```

## Services (`services/`)

### Meilisearch (`services/meilisearch/`)

```
services/meilisearch/
└── README.md               # Service-Dokumentation

**Docker Image:** getmeili/meilisearch:v1.7
**Port:** 7700 (intern)
**Volume:** meilisearch-data

**Features:**
- Hybrid Search (Vektor + Keyword)
- Document Chunk Storage
- Metadata Management
```

### Embedding API (`services/embedding-api/`)

```
services/embedding-api/
└── README.md               # Service-Dokumentation

**Docker Image:** ghcr.io/huggingface/text-embeddings-inference:cpu-1.5
**Port:** 8080 (intern)
**Model:** BAAI/bge-small-en-v1.5

**Features:**
- OpenAI-kompatible API
- Batch-Processing
- CPU & GPU Support
```

### LLM Inference (`services/llm-inference/`)

```
services/llm-inference/
└── README.md               # Service-Dokumentation

**Docker Image:** ghcr.io/ggerganov/llama.cpp:server
**Port:** 8080 (intern)
**Volume:** llm-models

**Features:**
- OpenAI-kompatible Chat API
- GGUF-Modell Support
- CPU-optimiert
```

## Docker (`docker/`)

```
docker/
├── docker-compose.yml      # Multi-Service Orchestrierung
├── frontend.Dockerfile     # Frontend Container-Build
├── backend.Dockerfile      # Backend Container-Build
├── .env.example            # Environment Template
└── .env                    # Aktuelle Environment (nicht committen)

**Netzwerk:** rag-net (bridge)
**Volumes:**
- meilisearch-data
- llm-models
```

## Dokumentation (`docs/`)

```
docs/
├── README.md               # Hauptdokumentation
├── ARCHITECTURE.md         # System-Architektur
├── DEPLOYMENT.md           # Deployment-Anleitungen
└── PROJECT_STRUCTURE.md    # Diese Datei

**Themen:**
- Getting Started
- Architektur-Diagramme
- RAG-Pipeline Details
- Deployment-Optionen
- Performance-Tuning
- Troubleshooting
```

## Datenfluss

### Dokumenten-Upload

```
Frontend (Upload)
    ↓
Backend (/api/documents)
    ↓
Dokument-Chunking
    ↓
Embedding API (Batch-Embeddings)
    ↓
Meilisearch (Store Chunks + Vectors)
```

### Chat-Query

```
Frontend (User Query)
    ↓
Backend (/api/chat)
    ↓
Embedding API (Query Embedding)
    ↓
Meilisearch (Hybrid Search)
    ↓
Backend (Context Assembly)
    ↓
LLM Inference (Generate Response)
    ↓
Frontend (Streaming Response)
```

## Build-Artefakte (nicht committet)

```
# Node.js / Frontend
node_modules/
apps/*/node_modules/
apps/frontend/dist/
*.tsbuildinfo

# Rust / Backend
apps/backend-orchestrator/target/
Cargo.lock

# Docker
.env
docker/.env

# IDE
.vscode/*
.idea/
```

## Wichtige Dateien

| Datei | Zweck |
|-------|-------|
| `pnpm-workspace.yaml` | Definiert Workspace-Packages |
| `package.json` (root) | Monorepo Scripts & Metadaten |
| `Makefile` | Convenience-Kommandos |
| `docker-compose.yml` | Service-Orchestrierung |
| `Cargo.toml` | Rust Dependencies |
| `vite.config.ts` | Frontend Build-Config |
| `.gitignore` | Git Ignore-Rules |

## Development Workflow

### 1. Neues Feature im Frontend

```bash
cd apps/frontend
# Feature entwickeln
pnpm dev
# Tests schreiben
pnpm test
# Build prüfen
pnpm build
```

### 2. Backend-Änderungen

```bash
cd apps/backend-orchestrator
# Feature entwickeln
cargo run
# Tests ausführen
cargo test
# Release-Build
cargo build --release
```

### 3. Docker-Updates

```bash
# Services neu bauen
docker-compose -f docker/docker-compose.yml build

# Services neu starten
docker-compose -f docker/docker-compose.yml up -d

# Logs prüfen
docker-compose -f docker/docker-compose.yml logs -f
```

## Package Management

### Pnpm Workspaces

```bash
# Dependency zu Frontend hinzufügen
pnpm --filter @rag-assistant/frontend add vue-query

# Dependency zu Root hinzufügen (shared)
pnpm add -w typescript

# Alle Packages installieren
pnpm install

# Workspace-übergreifende Commands
pnpm -r build  # build all packages
```

### Rust Cargo

```bash
# Dependency hinzufügen
cd apps/backend-orchestrator
cargo add serde_json

# Build
cargo build --release

# Tests
cargo test
```

## Skalierung der Struktur

### Neue App hinzufügen

```bash
mkdir apps/new-app
cd apps/new-app
npm init -y
# Name auf @rag-assistant/new-app setzen
```

### Neuer Service

```bash
mkdir services/new-service
# README.md mit Service-Beschreibung
# In docker-compose.yml eintragen
```

### Shared Package erstellen

```bash
mkdir packages/shared
cd packages/shared
npm init -y
# Name: @rag-assistant/shared
# In pnpm-workspace.yaml eintragen: 'packages/*'
```

## Umgebungsvariablen

### Frontend (`.env.local`)

```env
VITE_API_URL=http://localhost:8080
```

### Backend (`.env`)

```env
RUST_LOG=info
MEILISEARCH_URL=http://meilisearch:7700
MEILISEARCH_API_KEY=your_key
EMBEDDING_API_URL=http://embedding-api:8080
LLM_API_URL=http://llm-inference:8080
```

### Docker (`docker/.env`)

```env
MEILISEARCH_API_KEY=your_key
EMBEDDING_MODEL=BAAI/bge-small-en-v1.5
```

## Best Practices

1. **Dependencies:** Shared Dependencies im Root, spezifische in den jeweiligen Apps
2. **Environment:** Nie `.env` Dateien committen, immer `.env.example` pflegen
3. **Versioning:** Workspace-Versionen synchron halten
4. **Documentation:** READMEs in jedem Service/App aktuell halten
5. **Git:** Aussagekräftige Commit-Messages mit Workspace-Prefix
6. **Testing:** Tests in jedem Workspace ausführbar machen
7. **Docker:** Images regelmäßig updaten und neu bauen

## Nächste Schritte

- [ ] Shared TypeScript-Types Package erstellen
- [ ] End-to-End Tests hinzufügen
- [ ] CI/CD Pipeline einrichten
- [ ] Monitoring & Logging-Stack
- [ ] API-Dokumentation (OpenAPI/Swagger)
