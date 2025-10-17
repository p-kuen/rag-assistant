# Embedding API Service

Text Embedding Service basierend auf HuggingFace Text Embeddings Inference (TEI).

## Model

- **EmbeddingGemma** oder alternatives kompaktes Embedding-Modell
- OpenAI-kompatible API
- Optimiert für geringe Latenz

## Verwendung

Der Service stellt einen `/embeddings` Endpoint bereit, der mit der OpenAI Embeddings API kompatibel ist.

### Beispiel Request

```json
{
  "input": "Text der embedded werden soll",
  "model": "embedding-gemma"
}
```

### Beispiel Response

```json
{
  "data": [
    {
      "embedding": [0.123, -0.456, ...],
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
