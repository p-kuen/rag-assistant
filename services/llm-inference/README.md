# LLM Inference Service

Hochperformanter LLM Inference Service basierend auf **vanilla llama.cpp Server** für maximale Ressourceneffizienz und Performance. Dieser Service implementiert die OpenAI-kompatible Chat Completions API und ist speziell für den On-Premise-Betrieb optimiert.

## 🚀 Architektur & Performance

- **Engine**: Vanilla llama.cpp Server (bis zu 30% schneller als Ollama)
- **Format**: GGUF-quantisierte Modelle für optimale Speichereffizienz
- **API**: OpenAI-kompatible Chat Completions API
- **Optimierung**: CPU/GPU-optimiert mit Hardware-spezifischen Optimierungen

## 📋 Modell-Empfehlungen

### Empfohlene Modelle (Ressourceneffizienz)

| Modell | Parameter | Größe | RAM-Bedarf | Empfohlen für |
|--------|-----------|-------|------------|---------------|
| **Gemma-2-2B** | 2B | ~1.5GB | 2-4GB | Allgemeine Aufgaben, hohe Geschwindigkeit |
| **Phi-3-mini** | 3.8B | ~2.3GB | 3-5GB | Ausgewogene Performance/Qualität |
| **Qwen-2.5-3B** | 3B | ~1.9GB | 3-5GB | Mehrsprachig, gute Qualität |

### Modell-Download

Das Modell wird automatisch beim ersten Start heruntergeladen:

```bash
# Manueller Download (optional)
docker run --rm -v llm-models:/app/models \
  ghcr.io/ggerganov/llama.cpp:server \
  /bin/bash -c "cd /app && ./download_model.sh"
```

## ⚙️ Konfiguration

### Umgebungsvariablen

```bash
# .env Datei im docker/ Verzeichnis
# No API key needed for on-premise deployment
LLAMA_THREADS=4                    # CPU Threads (Standard: Anzahl CPU-Kerne)
LLAMA_GPU_LAYERS=0                 # GPU Layer (0 = CPU-only)
```

### Ressourcen-Optimierung

```yaml
# docker-compose.yml Ressourcenlimits
deploy:
  resources:
    limits:
      cpus: "4.0"      # Max. 4 CPU-Kerne
      memory: 8G       # Max. 8GB RAM
    reservations:
      cpus: "2.0"      # Min. 2 CPU-Kerne
      memory: 4G       # Min. 4GB RAM
```

## 🔧 API-Verwendung

### Chat Completions Request

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_api_key" \
  -d '{
    "model": "gemma-2-2b-it",
    "messages": [
      {
        "role": "system",
        "content": "Du bist ein hilfreicher Assistent für RAG-Systeme."
      },
      {
        "role": "user",
        "content": "Erkläre mir Retrieval-Augmented Generation."
      }
    ],
    "temperature": 0.7,
    "max_tokens": 500,
    "stream": false
  }'
```

### Streaming Response

```bash
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_api_key" \
  -d '{
    "model": "gemma-2-2b-it",
    "messages": [
      {
        "role": "user",
        "content": "Was ist RAG?"
      }
    ],
    "stream": true
  }'
```

### Beispiel Response

```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "gemma-2-2b-it",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "RAG (Retrieval-Augmented Generation) ist eine Technik..."
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 50,
    "total_tokens": 70
  }
}
```

## 🚀 Deployment

### 1. Service starten

```bash
# Alle Services starten
docker-compose -f docker/docker-compose.yml up -d

# Nur LLM Service starten
docker-compose -f docker/docker-compose.yml up -d llm-inference
```

### 2. Logs überwachen

```bash
# Alle Logs
docker-compose -f docker/docker-compose.yml logs -f

# Nur LLM Service Logs
docker-compose -f docker/docker-compose.yml logs -f llm-inference
```

### 3. Health Check

```bash
# Service Status prüfen
curl http://localhost:8080/health

# API Verfügbarkeit testen
curl http://localhost:8080/v1/models
```

## 🔧 Performance-Tuning

### CPU-Optimierung

```bash
# Threads basierend auf CPU-Kernen setzen
export LLAMA_THREADS=$(nproc)

# Spezifische CPU-Features aktivieren
export LLAMA_CUBLAS=1
export LLAMA_CUDA=1
```

### GPU-Unterstützung (Optional)

```yaml
# docker-compose.yml - GPU aktivieren
deploy:
  resources:
    reservations:
      devices:
        - driver: nvidia
          count: 1
          capabilities: [gpu]
```

```bash
# GPU Layer konfigurieren
export LLAMA_GPU_LAYERS=20  # Anzahl Layer auf GPU
```

### Memory-Optimierung

```bash
# Memory Lock aktivieren (verhindert Swapping)
--mlock

# Memory Mapping deaktivieren (bessere Performance)
--no-mmap

# Continuous Batching aktivieren
--cont-batching
```

## 📊 Monitoring

### Metriken abrufen

```bash
# Server Metriken
curl http://localhost:8080/metrics

# Model Information
curl http://localhost:8080/v1/models
```

### Performance-Monitoring

```bash
# Container Ressourcen überwachen
docker stats rag-llm-inference

# Detaillierte Logs
docker-compose logs -f llm-inference | grep -E "(tokens/sec|memory|gpu)"
```

## 🛠️ Troubleshooting

### Häufige Probleme

1. **Modell nicht gefunden**
   ```bash
   # Modell manuell herunterladen
   docker exec rag-llm-inference /app/download_model.sh
   ```

2. **Out of Memory**
   ```bash
   # Memory Limits erhöhen
   # In docker-compose.yml: memory: 12G
   ```

3. **Langsame Performance**
   ```bash
   # Threads optimieren
   export LLAMA_THREADS=$(nproc)
   
   # GPU aktivieren (falls verfügbar)
   export LLAMA_GPU_LAYERS=20
   ```

### Debug-Modus

```bash
# Detaillierte Logs aktivieren
docker-compose -f docker/docker-compose.yml up llm-inference

# Container Shell öffnen
docker exec -it rag-llm-inference /bin/bash
```

## 📚 Weitere Ressourcen

- [llama.cpp GitHub](https://github.com/ggml-org/llama.cpp)
- [OpenAI API Dokumentation](https://platform.openai.com/docs/api-reference)
- [GGUF Model Format](https://github.com/ggerganov/ggml/blob/master/docs/gguf.md)
- [Performance Benchmarks](https://github.com/ggerganov/llama.cpp#performance)
