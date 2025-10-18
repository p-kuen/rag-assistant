# Meilisearch Service

Der Meilisearch Service ist die zentrale Retrieval-Komponente der RAG-Pipeline. Er kombiniert Vektor- und Volltextsuche in einer hochperformanten, selbst gehosteten Suchmaschine.

## Übersicht

Meilisearch dient als:
- **Hybrid Search Engine**: Kombiniert semantische Vektorsuche mit robuster Keyword-Suche
- **Document Store**: Speichert Dokument-Chunks mit Metadaten
- **Context Gatekeeper**: Filtert und rankt relevante Inhalte für RAG

## Architektur

### Hybrid Search Strategy
- **Vektor-Suche**: Semantische Ähnlichkeit basierend auf EmbeddingGemma-300m
- **Volltext-Suche**: Keyword-Matching mit Typo-Toleranz
- **Score Normalization**: Harmonisierung beider Suchmethoden
- **Bucket Sorting**: Intelligente Relevanz-Bewertung

### Index-Struktur
```json
{
  "id": "unique_chunk_id",
  "content": "Dokumenten-Chunk Inhalt",
  "title": "Dokument-Titel",
  "metadata": "Zusätzliche Metadaten",
  "hierarchy_lvl1": "Kapitel/Überschrift Level 1",
  "hierarchy_lvl2": "Kapitel/Überschrift Level 2", 
  "hierarchy_lvl3": "Kapitel/Überschrift Level 3",
  "document_type": "markdown|pdf",
  "author": "Dokument-Autor",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "tags": ["tag1", "tag2"],
  "source_file": "original_filename.md",
  "chunk_index": 0,
  "relevance_score": 0.95
}
```

## Installation und Setup

### 1. Konfiguration
```bash
# Konfigurationsdatei kopieren
cp config.env.example config.env

# Werte anpassen (optional)
nano config.env
```

### 2. Docker Build und Run
```bash
# Service bauen
make build

# Service starten
make run

# Vollständiges Setup (build + run + init)
make setup
```

### 3. Index-Initialisierung
```bash
# Manuelle Initialisierung
./init_meilisearch.sh

# Oder über Makefile
make init-index
```

## Verwendung

### API-Endpoints

#### Health Check
```bash
curl http://localhost:7700/health
```

#### Dokument hinzufügen
```bash
curl -X POST 'http://localhost:7700/indexes/rag_documents/documents' \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "id": "doc_1_chunk_1",
      "content": "Dies ist ein Beispiel-Chunk...",
      "title": "Beispiel-Dokument",
      "document_type": "markdown",
      "author": "Max Mustermann",
      "created_at": "2024-01-01T00:00:00Z",
      "tags": ["tutorial", "example"]
    }
  ]'
```

#### Hybrid Search
```bash
curl -X POST 'http://localhost:7700/indexes/rag_documents/search' \
  -H 'Content-Type: application/json' \
  -d '{
    "q": "Beispiel-Suche",
    "hybrid": {
      "semanticRatio": 0.5,
      "embedder": "default"
    },
    "filter": "document_type = markdown",
    "limit": 10
  }'
```

#### Gefilterte Suche
```bash
curl -X POST 'http://localhost:7700/indexes/rag_documents/search' \
  -H 'Content-Type: application/json' \
  -d '{
    "q": "Suchbegriff",
    "filter": "author = \"Max Mustermann\" AND created_at > 2024-01-01",
    "sort": ["created_at:desc"],
    "limit": 5
  }'
```

### Rust Backend Integration

```rust
use meilisearch_sdk::{client::*, indexes::*, search::*};

// Client initialisieren
let client = Client::new("http://localhost:7700", Some("master_key"));

// Index referenzieren
let index = client.index("rag_documents");

// Hybrid Search ausführen
let search_result = index
    .search()
    .with_query("user query")
    .with_hybrid(Hybrid {
        semantic_ratio: 0.5,
        embedder: "default".to_string(),
    })
    .with_filter("document_type = markdown")
    .with_limit(10)
    .execute::<serde_json::Value>()
    .await?;
```

## Konfiguration

### Umgebungsvariablen
- `MEILI_MASTER_KEY`: Master-Key für Authentifizierung
- `MEILI_ENV`: Umgebung (development/production)
- `MEILI_DB_PATH`: Datenbank-Pfad
- `MEILI_HTTP_ADDR`: HTTP-Server Adresse
- `MEILI_LOG_LEVEL`: Log-Level
- `MEILI_MAX_INDEXING_MEMORY`: Max. Speicher für Indexierung
- `MEILI_MAX_INDEXING_THREADS`: Max. Indexierungs-Threads

### Embedder-Konfiguration
Der Service ist für die Integration mit dem EmbeddingGemma-300m Modell konfiguriert:
- **Modell**: google/embeddinggemma-300m
- **Dimensionen**: 768 (MRL-fähig: 256-512)
- **API**: OpenAI-kompatibel über embedding-api Service
- **Document Template**: Kombiniert Hierarchie + Content für bessere Embeddings

## Management

### Makefile-Befehle
```bash
make help          # Hilfe anzeigen
make build         # Docker Image bauen
make run           # Container starten
make stop          # Container stoppen
make clean         # Container und Volumes entfernen
make logs          # Logs anzeigen
make health        # Health Check
make init-index    # Index initialisieren
make test          # API-Tests
make setup         # Vollständiges Setup
make backup        # Backup erstellen
make reset-index   # Index zurücksetzen (VORSICHT!)
```

### Wartung
```bash
# Logs verfolgen
make logs

# Health Status prüfen
make health

# Backup erstellen
make backup

# Index-Statistiken
./init_meilisearch.sh info
```

## Performance-Optimierung

### Ressourcen-Limits
- **Memory**: 2GB für Indexierung
- **Threads**: 4 parallele Indexierungs-Threads
- **Payload**: 100MB max. Request-Größe

### Index-Optimierung
- **Searchable Attributes**: Content, Title, Metadata, Hierarchy
- **Filterable Attributes**: Document-Type, Author, Tags, Dates
- **Sortable Attributes**: Created-At, Relevance-Score
- **Ranking Rules**: Words → Typo → Proximity → Attribute → Sort → Exactness

### Vektor-Optimierung
- **MRL Support**: Dimensionen zur Laufzeit anpassbar (768 → 512 → 256)
- **Document Template**: Kontextuelle Embeddings durch Hierarchie-Integration
- **Batch Processing**: Bis zu 1000 Dokumente pro Batch

## Troubleshooting

### Häufige Probleme

#### Service startet nicht
```bash
# Logs prüfen
make logs

# Container-Status
docker ps -a | grep meilisearch

# Volumes prüfen
docker volume ls | grep meilisearch
```

#### Index-Initialisierung fehlgeschlagen
```bash
# Service-Status prüfen
make health

# Manuelle Initialisierung
./init_meilisearch.sh

# Index zurücksetzen
make reset-index
```

#### Embedder-Verbindung fehlgeschlagen
```bash
# Embedding-API Status prüfen
curl http://embedding-api:8080/health

# Netzwerk-Konnektivität testen
docker network ls | grep rag-net
```

### Log-Analyse
```bash
# Detaillierte Logs
docker logs rag-meilisearch --tail 100

# Logs in Echtzeit
docker logs -f rag-meilisearch
```

## Integration mit RAG-Pipeline

### Ingestion-Workflow
1. **Rust Backend** → Dokumenten-Parsing & Chunking
2. **Embedding Service** → Vektor-Generierung
3. **Meilisearch** → Indexierung mit Metadaten

### Retrieval-Workflow
1. **User Query** → Rust Backend
2. **Hybrid Search** → Meilisearch (Vektor + Keyword)
3. **Context Compilation** → Rust Backend
4. **LLM Generation** → LLM Inference Service

### Metadaten-Nutzung
- **Filtering**: Dokument-Typ, Autor, Datum, Tags
- **Sorting**: Relevanz, Datum, Chunk-Index
- **Source Attribution**: Dateiname, Hierarchie, Position

## Sicherheit

### Produktions-Setup
```bash
# Master-Key generieren
openssl rand -base64 32

# In config.env setzen
MEILI_MASTER_KEY=your_secure_key_here
MEILI_ENV=production
```

### CORS-Konfiguration
- **Development**: Alle Origins erlaubt
- **Production**: Spezifische Domains konfigurieren

## Monitoring

### Health Checks
- **Endpoint**: `GET /health`
- **Interval**: 30s
- **Timeout**: 10s
- **Retries**: 3

### Metriken
- **Index-Größe**: Anzahl Dokumente, Speicherverbrauch
- **Query-Performance**: Latenz, Durchsatz
- **Embedder-Status**: API-Verfügbarkeit, Response-Zeit

## Referenzen

- [Meilisearch Documentation](https://www.meilisearch.com/docs)
- [Hybrid Search Guide](https://www.meilisearch.com/blog/mastering-rag)
- [RAG Best Practices](https://www.meilisearch.com/blog/how-to-build-rag)
- [EmbeddingGemma Model](https://huggingface.co/blog/embeddinggemma)
