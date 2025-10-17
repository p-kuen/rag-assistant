# Architektur-Übersicht

## Systemarchitektur

Die RAG-Plattform folgt einer **Microservices-Architektur** mit einem zentralen Orchestrator-Backend.

### Architekturprinzipien

1. **Modularität:** Jeder Service hat eine klare, abgegrenzte Verantwortung
2. **Skalierbarkeit:** Services können unabhängig voneinander skaliert werden
3. **Ressourcenoptimierung:** Kompakte Modelle und effiziente Container
4. **OpenAI-Kompatibilität:** Standard-APIs für Embedding und LLM

## Komponenten-Diagramm

```
┌──────────────────────────────────────────────────────────────┐
│                         Frontend Layer                        │
│  ┌────────────────┐              ┌───────────────────────┐   │
│  │ Chat Interface │              │ Wissensverwaltung UI  │   │
│  │   (/chat)      │              │ (/admin/knowledge)    │   │
│  └────────────────┘              └───────────────────────┘   │
└──────────────────────────────────┬───────────────────────────┘
                                   │ HTTP/REST
┌──────────────────────────────────┴───────────────────────────┐
│                    Backend Orchestrator (Rust)                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ API Gateway  │  │ RAG Pipeline │  │ Document Chunker │   │
│  └──────────────┘  └──────────────┘  └──────────────────┘   │
└──────┬────────────────┬────────────────────┬─────────────────┘
       │                │                    │
       │ HTTP           │ HTTP               │ HTTP
       ▼                ▼                    ▼
┌─────────────┐  ┌──────────────┐    ┌──────────────┐
│ Meilisearch │  │ Embedding API│    │ LLM Inference│
│             │  │   (TEI)      │    │ (llama.cpp)  │
│ Hybrid      │  │              │    │              │
│ Search      │  │ bge-small    │    │ Gemma-2-2B   │
└─────────────┘  └──────────────┘    └──────────────┘
      │
      ▼
┌─────────────┐
│ Persistent  │
│   Volume    │
└─────────────┘
```

## RAG-Pipeline Ablauf

### 1. Dokument-Indexierung

```
User Upload
    │
    ▼
┌────────────────────┐
│ Backend: Receive   │
│ Document + Metadata│
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Chunking Strategy  │
│ - Fixed Size       │
│ - Semantic Split   │
│ - Hierarchical     │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Generate Embeddings│ ──► Embedding API
│ for each chunk     │     (TEI)
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Store in Meilisearch│
│ - Vector           │
│ - Text             │
│ - Metadata         │
└────────────────────┘
```

### 2. Query-Verarbeitung (Chat)

```
User Query
    │
    ▼
┌────────────────────┐
│ Backend: Parse     │
│ Query              │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Generate Query     │ ──► Embedding API
│ Embedding          │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Hybrid Search      │ ──► Meilisearch
│ - Vector Similarity│
│ - Keyword Match    │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Retrieve Top-K     │
│ Relevant Chunks    │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Build Prompt with  │
│ Context + Query    │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ LLM Inference      │ ──► llama.cpp
│ Generate Response  │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Stream Response    │
│ to Frontend        │
└────────────────────┘
```

## Service-Details

### Backend Orchestrator (Rust)

**Technologie-Stack:**
- **Framework:** Axum (async web framework)
- **Runtime:** Tokio (async runtime)
- **HTTP Client:** Reqwest
- **Serialization:** Serde

**Hauptverantwortlichkeiten:**
1. API Gateway für Frontend-Anfragen
2. RAG-Pipeline Orchestrierung
3. Dokument-Chunking
4. Service-zu-Service-Kommunikation
5. Error Handling und Retry-Logik

**Warum Rust?**
- Hohe Performance bei geringem Speicher-Overhead
- Type Safety für robuste Service-Integration
- Async/Await für effiziente I/O-Operationen
- Zero-Cost Abstractions

### Frontend (VueJS)

**Technologie-Stack:**
- **Framework:** Vue 3 (Composition API)
- **Build Tool:** Vite
- **Styling:** Tailwind CSS v4
- **Routing:** Vue Router
- **Icons:** Lucide Vue

**Features:**
- Server-Sent Events (SSE) für Streaming-Responses
- Markdown-Rendering für Chat-Nachrichten
- Drag & Drop Dokument-Upload
- Real-time Task-Status-Updates

### Meilisearch

**Warum Meilisearch?**
- Native Unterstützung für Hybrid Search (Vektor + Keyword)
- Geringe Ressourcenanforderungen
- Einfache REST API
- Built-in Relevanz-Scoring
- Schnelle Indexierung

**Index-Schema:**
```json
{
  "id": "chunk-uuid",
  "document_id": "doc-uuid",
  "content": "chunk text content",
  "embedding": [0.123, -0.456, ...],
  "metadata": {
    "title": "Document Title",
    "filename": "document.pdf",
    "hierarchy_path": "chapter1/section2",
    "created_at": "2025-01-01T00:00:00Z"
  }
}
```

### Embedding API (TEI)

**HuggingFace Text Embeddings Inference:**
- Optimiert für schnelle Embedding-Generierung
- Batch-Processing Support
- OpenAI-kompatible API
- CPU und GPU Support

**Default Model:** `BAAI/bge-small-en-v1.5`
- Dimension: 384
- Performance: ~1ms pro Embedding (CPU)
- Multilingual Support

**Alternativen:**
- `sentence-transformers/all-MiniLM-L6-v2` (leichter)
- `BAAI/bge-base-en-v1.5` (besser, aber langsamer)

### LLM Inference (llama.cpp)

**Warum llama.cpp?**
- Optimiert für CPU-Inferenz
- Quantisierte Modelle (GGUF-Format)
- OpenAI-kompatible Chat API
- Geringe Latenz auch ohne GPU

**Empfohlene Modelle:**

| Modell | Größe (Q4) | RAM | Qualität | Use Case |
|--------|------------|-----|----------|----------|
| Gemma-2-2B-IT | ~1.5GB | 4GB | Gut | Allgemein |
| Phi-3-mini | ~2.3GB | 4GB | Sehr gut | Reasoning |
| Qwen-2.5-3B | ~2.0GB | 4GB | Sehr gut | Multilingual |

**Context-Strategie:**
- Max Context: 4096 Tokens
- Sliding Window für lange Dokumente
- Automatisches Context-Truncation

## Datenfluss

### Persistenz

```
┌─────────────────┐
│ Docker Volumes  │
├─────────────────┤
│ meilisearch-    │ ◄── Meilisearch Daten
│   data          │     (Chunks, Vectors, Metadata)
│                 │
│ llm-models      │ ◄── LLM GGUF Modelle
│                 │     (model.gguf)
└─────────────────┘
```

### Netzwerk

```
┌──────────────────────────────────────────┐
│            rag-net (Bridge)              │
│                                          │
│  frontend:5173 ◄──► backend:8080        │
│                          │               │
│                          ├──► meilisearch:7700
│                          ├──► embedding-api:8080
│                          └──► llm-inference:8080
└──────────────────────────────────────────┘
     ▲                    ▲
     │                    │
  Port 5173            Port 8080
  (extern)             (extern)
```

## Skalierungsoptionen

### Horizontal Scaling

1. **Meilisearch:** Multi-Node Cluster für größere Datenmengen
2. **Embedding API:** Load Balancer vor mehreren TEI-Instanzen
3. **LLM Inference:** Queue-basiertes Routing zu mehreren llama.cpp Servern

### Vertical Scaling

1. **GPU Support:** Beide AI-Services unterstützen CUDA
2. **Mehr RAM:** Größere Context Windows für LLM
3. **Mehr CPU:** Parallelisierung in Backend Orchestrator

### Performance-Optimierungen

1. **Caching:**
   - Query-Embedding Cache
   - Häufige Chunks im Memory
   - LLM Response Cache

2. **Batching:**
   - Batch Embedding-Requests
   - Chunk-Processing in Batches

3. **Asynchrone Verarbeitung:**
   - Background Jobs für Indexierung
   - Stream Processing für große Dokumente

## Monitoring & Observability

**Health Checks:**
- Alle Services exponieren `/health` Endpoints
- Docker Health Checks konfiguriert
- Automatic Restart bei Failures

**Logs:**
- Strukturierte Logs (JSON)
- Log-Level: DEBUG, INFO, WARN, ERROR
- Zugriff via `docker-compose logs -f`

**Metriken (zukünftig):**
- Prometheus Exporter
- Grafana Dashboards
- Request Latencies
- Token-Usage Tracking

## Sicherheit

**Netzwerk-Isolation:**
- Interne Services nicht extern erreichbar
- Frontend ↔ Backend via CORS Policy
- Backend ↔ Services via internes Netzwerk

**Authentifizierung:**
- Meilisearch Master Key
- Zukünftig: JWT für Frontend-API

**Daten-Persistenz:**
- Encrypted Volumes (optional)
- Backup-Strategien für Meilisearch
- Model Checksums für Integrität
