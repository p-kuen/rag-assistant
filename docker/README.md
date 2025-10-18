# Docker Configuration

Dieses Verzeichnis enthält alle Docker-Konfigurationsdateien für die RAG-Plattform.

## Dateien

- `docker-compose.yml` - Hauptorchestrierung aller Services
- `frontend.Dockerfile` - VueJS Frontend Build
- `backend.Dockerfile` - Rust Backend Build  
- `llm-inference.Dockerfile` - llama.cpp Service Build
- `env.example` - Umgebungsvariablen Template

## Setup

### 1. Umgebungsvariablen konfigurieren

```bash
# Template kopieren
cp docker/.env.example docker/.env

# Werte anpassen
nano docker/.env
```

### 2. Services starten

```bash
# Alle Services starten
docker compose -f docker/docker-compose.yml up -d

# Logs anzeigen
docker compose -f docker/docker-compose.yml logs -f

# Services stoppen
docker compose -f docker/docker-compose.yml down
```

## Umgebungsvariablen

Alle Konfiguration erfolgt über die zentrale `.env` Datei im `docker/` Verzeichnis:

- `MEILISEARCH_API_KEY` - Master-Key für Meilisearch
- `EMBEDDING_MODEL` - HuggingFace Embedding-Modell
- `LLAMA_THREADS` - CPU-Threads für llama.cpp
- `LLAMA_GPU_LAYERS` - GPU-Layer (0 = CPU-only)

## Services

- **Frontend**: http://localhost:5173
- **Backend**: http://localhost:8080
- **Meilisearch**: Intern (Port 7700)
- **Embedding API**: Intern (Port 8080)
- **LLM Inference**: Intern (Port 8080)
