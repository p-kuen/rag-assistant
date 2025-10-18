# RAG Assistant - Ressourcenoptimierte Fullstack RAG-Plattform

Ein modulares, Docker-basiertes RAG (Retrieval-Augmented Generation) System mit VueJS Frontend und Rust Backend.

[![Architecture](https://img.shields.io/badge/Architecture-Microservices-blue)](./docs/ARCHITECTURE.md)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?logo=docker)](./docker/docker-compose.yml)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)

## 🚀 Features

- ✅ **Chat-Assistent**: Interaktive RAG-basierte Konversation mit Quellenangaben
- ✅ **Wissensverwaltung**: Dokument-Upload und manuelle Markdown-Eingabe
- ✅ **Hybrid Search**: Kombination aus Vektor- und Keyword-Suche via Meilisearch
- ✅ **Streaming Responses**: Real-time Token-Streaming vom LLM
- ✅ **Ressourcenoptimiert**: Kompakte Modelle und effiziente Container-Orchestrierung
- ✅ **OpenAI-kompatibel**: Standard-APIs für Embedding und LLM
- ✅ **Vollständige RAG-Pipeline**: Parsing, Chunking, Embedding, Indexierung, Retrieval, Generation

## 📁 Monorepo-Struktur

```
rag-assistant/
├── apps/
│   ├── frontend/              # VueJS 3 + TypeScript + Vite
│   └── backend-orchestrator/  # Rust (Axum) Backend
├── services/
│   ├── meilisearch/          # Vektor- & Metadaten-Speicher
│   ├── embedding-api/        # Text Embeddings (TEI)
│   └── llm-inference/        # LLM Service (llama.cpp)
├── docker/
│   ├── docker-compose.yml    # Orchestrierung aller Services
│   └── *.Dockerfile
└── docs/                      # Umfassende Dokumentation
```

## 🏗️ Architektur

```
┌─────────────┐
│  Frontend   │ :5173 (VueJS)
│  (Vue 3)    │
└──────┬──────┘
       │ REST API
       ▼
┌─────────────┐
│   Backend   │ :8080 (Rust)
│ Orchestrator│
└──────┬──────┘
       │
   ┌───┼────────┬─────────┐
   ▼   ▼        ▼         ▼
┌─────┐┌──────┐┌────────┐
│Meili││Embed ││  LLM   │
│srch ││ API  ││Inferenc│
└─────┘└──────┘└────────┘
```

Siehe [ARCHITECTURE.md](./docs/ARCHITECTURE.md) für Details.

## 🚀 Quick Start

### Voraussetzungen

- **Node.js** >= 18.0.0
- **Pnpm** >= 8.0.0
- **Docker** und **Docker Compose**
- **Rust** >= 1.75 (optional, für Backend-Entwicklung)

### 1. Installation

```bash
# Repository klonen
git clone <repository-url>
cd rag-assistant

# Dependencies installieren
pnpm install
```

### 2. LLM Modell herunterladen

```bash
# Gemma-2-2B quantisiert (~1.5GB)
docker run -v rag-assistant_llm-models:/models alpine sh -c \
  "apk add --no-cache wget && \
   wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf \
   -O /models/model.gguf"
```

### 3. Environment konfigurieren

```bash
# .env Datei erstellen
cp docker/.env.example docker/.env

# API Keys setzen
nano docker/.env
```

### 4. Services starten

```bash
# Alle Docker Services starten
pnpm docker:up

# Logs verfolgen
pnpm docker:logs
```

### 5. Development

```bash
# Frontend (http://localhost:5173)
pnpm dev:frontend

# Backend (http://localhost:8080)
pnpm dev:backend
```

## 📦 Verfügbare Scripts

```bash
# Development
pnpm dev              # Alle Apps im Dev-Modus
pnpm dev:frontend     # Nur Frontend
pnpm dev:backend      # Nur Backend

# Build
pnpm build            # Alle Apps bauen
pnpm build:frontend   # Frontend Production Build
pnpm build:backend    # Backend Release Build

# Docker
pnpm docker:up        # Services starten
pnpm docker:down      # Services stoppen
pnpm docker:logs      # Logs anzeigen
```

## 🛠️ Technologie-Stack

### Frontend
- **Vue 3** - Progressive JavaScript Framework
- **TypeScript** - Type Safety
- **Vite** - Build Tool & Dev Server
- **Tailwind CSS v4** - Styling
- **Vue Router** - Routing

### Backend
- **Rust** - Systems Programming Language
- **Axum** - Async Web Framework
- **Tokio** - Async Runtime
- **Reqwest** - HTTP Client

### Services
- **Meilisearch** - Hybrid Search Engine
- **HuggingFace TEI** - Embedding API
- **llama.cpp** - LLM Inference

## 📚 Dokumentation

- **[Hauptdokumentation](./docs/README.md)** - Umfassende Übersicht
- **[Architektur](./docs/ARCHITECTURE.md)** - System-Design & RAG-Pipeline
- **[Deployment](./docs/DEPLOYMENT.md)** - Deployment-Anleitungen & Tuning

## 🎯 Use Cases

- **Interne Wissensdatenbank**: Unternehmensdokumentation durchsuchbar machen
- **Customer Support**: FAQ-basierte Assistenzsysteme
- **Research Assistant**: Wissenschaftliche Paper-Analyse
- **Code Documentation**: Code-Repository-Abfragen

## 🔒 Sicherheit

- ✅ Interne Services sind nicht extern erreichbar
- ✅ Meilisearch API Key erforderlich
- ✅ Container laufen mit non-root User
- ✅ Docker Network Isolation

## 📊 Ressourcenanforderungen

| Service | Min. RAM | Min. CPU | Empfohlen |
|---------|----------|----------|-----------|
| Frontend | 512MB | 0.5 | 1GB / 1 CPU |
| Backend | 512MB | 0.5 | 2GB / 2 CPU |
| Meilisearch | 1GB | 0.5 | 4GB / 2 CPU |
| Embedding API | 2GB | 1.0 | 4GB / 2 CPU |
| LLM Inference | 4GB | 2.0 | 8GB / 4 CPU |

**Total:** Min. 8GB RAM, 4 CPU Cores

## 🔧 Troubleshooting

### Services starten nicht
```bash
# Status checken
docker-compose -f docker/docker-compose.yml ps

# Logs für spezifischen Service
docker-compose -f docker/docker-compose.yml logs llm-inference

# Service neu starten
docker-compose -f docker/docker-compose.yml restart llm-inference
```

### "Model not found" Error
Das LLM-Modell wurde nicht heruntergeladen. Siehe Quick Start Schritt 2.

### Port bereits belegt
Andere Anwendung nutzt Port 5173 oder 8080:
```bash
# Ports in docker-compose.yml ändern:
ports:
  - "5174:5173"  # Frontend auf 5174
  - "8081:8080"  # Backend auf 8081
```

### Out of Memory
Docker Memory-Limit erhöhen:
- **Docker Desktop:** Settings → Resources → Memory → Min. 8GB

### Frontend zeigt Fehler
1. Backend läuft nicht → `pnpm dev:backend` oder Docker-Services starten
2. CORS-Error → Backend API URL in `.env` prüfen

### Meilisearch Index nicht initialisiert
```bash
# Manuell initialisieren
docker exec rag-meilisearch /init_meilisearch.sh
```

## 🤝 Beitragen

1. Fork das Repository
2. Feature Branch erstellen (`git checkout -b feature/amazing-feature`)
3. Änderungen committen (`git commit -m '[frontend] Add amazing feature'`)
4. Branch pushen (`git push origin feature/amazing-feature`)
5. Pull Request öffnen

## 📝 Lizenz

[Ihre Lizenz hier - z.B. MIT]

## 🙏 Acknowledgments

- **Meilisearch** - Hybrid Search Engine
- **HuggingFace** - Text Embeddings Inference
- **llama.cpp** - Efficient LLM Inference
- **Vue.js Team** - Frontend Framework

---

**Built with ❤️ using Rust, Vue, and Open Source AI**
