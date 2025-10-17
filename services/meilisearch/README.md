# Meilisearch Service

Vektor- und Metadaten-Speicher für die RAG-Pipeline.

## Konfiguration

Meilisearch wird als Docker-Container ausgeführt und nutzt Persistent Volumes für Daten.

## Verwendete Features

- Hybrid Search (Vektor + Keyword)
- Speicherung von Document Chunks
- Metadaten für Retrieval

## Wichtige Endpoints

- `POST /indexes/{index_name}/search` - Hybrid Search
- `POST /indexes/{index_name}/documents` - Dokumente hinzufügen
- `GET /health` - Health Check
