# Embedding API Service

Hochperformanter Text Embedding Service basierend auf HuggingFace Text Embeddings Inference (TEI) mit EmbeddingGemma-Modell für ressourceneffiziente On-Premise RAG-Anwendungen.

## 🎯 Übersicht

Dieser Service implementiert die **Embedding-Komponente** der RAG-Plattform und nutzt **EmbeddingGemma** - ein hochoptimiertes, mehrsprachiges Embedding-Modell mit weniger als 500 Millionen Parametern. Der Service bietet eine OpenAI-kompatible API für nahtlose Integration in das Rust Backend.

## 🎯 Zweck und Aufgabe

### Was ist der Embedding API Service?

Der Embedding API Service ist ein **spezialisierter Microservice** innerhalb der RAG-Plattform, der ausschließlich für die **Vektorisierung von Textdokumenten** zuständig ist. Er wandelt menschlich lesbare Texte in mathematische Vektoren (Embeddings) um, die von Suchmaschinen wie Meilisearch für semantische Suche verwendet werden können.

### Hauptaufgaben des Services:

1. **Text-zu-Vektor-Transformation**: Konvertiert Dokumenten-Chunks in hochdimensionale Vektoren (768 Dimensionen bei EmbeddingGemma)
2. **Batch-Verarbeitung**: Verarbeitet mehrere Texte gleichzeitig für optimale Performance
3. **Semantische Repräsentation**: Erstellt Vektoren, die semantische Ähnlichkeiten zwischen Texten erfassen
4. **Ressourceneffizienz**: Nutzt kompakte, quantisierte Modelle für minimalen Speicherverbrauch

### Rolle im RAG-Workflow:

```
Dokument → [Rust Backend] → [Embedding API] → Vektor → [Meilisearch] → Hybrid-Suche
```

- **Ingestion-Phase**: Das Rust Backend sendet Dokumenten-Chunks an den Embedding Service
- **Vektorisierung**: Der Service generiert Embeddings für jeden Chunk
- **Indexierung**: Die Vektoren werden zusammen mit dem Originaltext in Meilisearch gespeichert
- **Retrieval-Phase**: Bei Benutzeranfragen werden ähnliche Vektoren in Meilisearch gefunden

### Warum ein separater Service?

1. **Skalierbarkeit**: Embedding-Generierung kann unabhängig von anderen Komponenten skaliert werden
2. **Ressourcen-Isolation**: Speicher- und CPU-intensive Vektorisierung läuft in isoliertem Container
3. **Wiederverwendbarkeit**: Andere Services können den Embedding Service ebenfalls nutzen
4. **Wartbarkeit**: Modell-Updates oder -Wechsel sind ohne Änderung des Backends möglich
5. **Performance**: Dedizierte Hardware-Optimierung für Embedding-Generierung

### Kernmerkmale

- **EmbeddingGemma 300M**: Hochperformantes, kompaktes Embedding-Modell
- **Matryoshka Representation Learning (MRL)**: Dynamische Vektordimensionen (128-768)
- **OpenAI-kompatible API**: Nahtlose Integration mit dem Rust Backend
- **Ressourceneffizient**: <200MB RAM bei quantisierten Modellen
- **Docker-optimiert**: Einfache Containerisierung und Orchestrierung

## 🚀 Schnellstart

### 1. Automatisches Setup

```bash
# Vollständiges Setup (empfohlen)
make setup

# Service starten
make up

# Service testen
make test
```

### 2. Manuelles Setup

```bash
# 1. Konfiguration erstellen
cp config.env.example config.env

# 2. Modell herunterladen
make download-model

# 3. Docker Image bauen
make build

# 4. Service starten
make up
```

## 📋 Verfügbare Befehle

```bash
make help          # Zeigt alle verfügbaren Befehle
make setup         # Vollständiges Setup (Modell + Config + Build)
make build         # Docker Image bauen
make up            # Service starten
make down          # Service stoppen
make restart       # Service neustarten
make logs          # Logs anzeigen
make test          # Service testen
make health        # Gesundheitsstatus prüfen
make status        # Service-Status anzeigen
make clean         # Alle Ressourcen entfernen
make config        # Aktuelle Konfiguration anzeigen
```

## ⚙️ Konfiguration

### Umgebungsvariablen

Die wichtigsten Konfigurationsoptionen in `config.env`:

```bash
# Modell-Konfiguration
MODEL_ID=google/embeddinggemma-300m
MODEL_REVISION=main

# Server-Konfiguration
HOST=0.0.0.0
PORT=80
MAX_BATCH_SIZE=32
MAX_BATCH_TOKENS=16384

# Embedding-Konfiguration
POOLING_MODE=mean
NORMALIZE_EMBEDDINGS=true
MAX_INPUT_LENGTH=512

# Performance-Tuning
NUM_WORKERS=1
MAX_CONCURRENT_REQUESTS=100
```

### Hardware-Optimierung

Der Service erkennt automatisch verfügbare Hardware und optimiert die Konfiguration:

- **CPU**: Reduzierte Batch-Größen für optimale Performance
- **GPU**: Erweiterte Batch-Größen für maximale Durchsatz
- **Automatische Erkennung**: NVIDIA, AMD oder CPU-only

## 🔌 API-Verwendung

### OpenAI-kompatible Embeddings API

Der Service stellt einen `/embeddings` Endpoint bereit, der vollständig mit der OpenAI Embeddings API kompatibel ist.

#### Beispiel Request

```bash
curl -X POST http://localhost:8081/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "input": "Text der embedded werden soll",
    "model": "embedding-gemma"
  }'
```

#### Beispiel Response

```json
{
  "data": [
    {
      "embedding": [0.123, -0.456, 0.789, ...],
      "index": 0
    }
  ],
  "model": "embedding-gemma",
  "usage": {
    "prompt_tokens": 10,
    "total_tokens": 10
  }
}
```

### Batch-Verarbeitung

```bash
curl -X POST http://localhost:8081/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "input": [
      "Erster Text für Embedding",
      "Zweiter Text für Embedding",
      "Dritter Text für Embedding"
    ],
    "model": "embedding-gemma"
  }'
```

### Health Check

```bash
curl http://localhost:8081/health
```

## 🏗️ Architektur-Integration

### Integration mit Rust Backend

Das Rust Backend nutzt den `openai4rs` Client für die Kommunikation:

```rust
use openai4rs::client::Client;
use openai4rs::embeddings::CreateEmbeddingRequest;

let client = Client::new("http://embedding-api:80", "dummy-key");
let request = CreateEmbeddingRequest {
    model: "embedding-gemma".to_string(),
    input: "Text to embed".to_string(),
    ..Default::default()
};

let response = client.embeddings().create(request).await?;
```

### Meilisearch Integration

Die generierten Embeddings werden direkt an Meilisearch für die Hybrid-Suche übertragen:

```rust
// Vektorisierung im Rust Backend
let embeddings = embedding_client.create_embedding(chunk_text).await?;

// Indexierung in Meilisearch
let document = Document {
    id: chunk_id,
    content: chunk_text,
    embedding: embeddings.data[0].embedding,
    metadata: chunk_metadata,
};

meilisearch_client.add_documents(&[document]).await?;
```

## 📊 Performance-Optimierung

### Matryoshka Representation Learning (MRL)

EmbeddingGemma unterstützt MRL, wodurch Vektordimensionen zur Laufzeit angepasst werden können:

- **Vollständige Dimension**: 768 (höchste Qualität)
- **Reduzierte Dimension**: 512, 256, 128 (bessere Performance)
- **Automatische Optimierung**: Basierend auf Hardware-Ressourcen

### Batch-Optimierung

Der Service optimiert automatisch Batch-Größen basierend auf:

- Verfügbare Hardware (CPU/GPU)
- Speicher-Ressourcen
- Netzwerk-Latenz
- Modell-Parameter

## 🔧 Entwicklung und Debugging

### Logs anzeigen

```bash
make logs
```

### Service-Status prüfen

```bash
make status
make health
```

### Lokale Entwicklung

```bash
# Service im Vordergrund starten
docker run --rm -it \
  --name embedding-api-dev \
  -p 8081:80 \
  --env-file config.env \
  rag-assistant/embedding-api:latest
```

## 🐳 Docker-Integration

### Docker Compose

Der Service ist in die Haupt-Docker-Compose-Konfiguration integriert:

```yaml
services:
  embedding-api:
    build: ./services/embedding-api
    ports:
      - "8081:80"
    environment:
      - MODEL_ID=google/embeddinggemma-300m
      - MAX_BATCH_SIZE=32
    networks:
      - rag-net
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '2.0'
```

### Ressourcen-Limits

Empfohlene Ressourcen-Limits für verschiedene Hardware-Konfigurationen:

- **Minimal (CPU-only)**: 1GB RAM, 1 CPU
- **Standard (CPU)**: 2GB RAM, 2 CPUs  
- **Optimiert (GPU)**: 4GB RAM, 4 CPUs + GPU

## 🚨 Troubleshooting

### Häufige Probleme

1. **Service startet nicht**
   ```bash
   make logs  # Logs prüfen
   make health  # Gesundheitsstatus prüfen
   ```

2. **Modell lädt nicht**
   ```bash
   make download-model  # Modell erneut herunterladen
   ```

3. **Performance-Probleme**
   ```bash
   make config  # Konfiguration prüfen
   # Batch-Größen in config.env anpassen
   ```

### Debug-Modus

```bash
# Debug-Logs aktivieren
export LOG_LEVEL=DEBUG
make restart
```

## 📚 Weitere Ressourcen

- [EmbeddingGemma Dokumentation](https://huggingface.co/blog/embeddinggemma)
- [Text Embeddings Inference (TEI)](https://huggingface.co/docs/text-embeddings-inference)
- [OpenAI Embeddings API](https://platform.openai.com/docs/api-reference/embeddings)
- [Meilisearch Vector Search](https://www.meilisearch.com/docs/learn/advanced/vector_search)
