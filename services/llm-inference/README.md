# LLM Inference Service

LLM Inference Service basierend auf llama.cpp Server.

## Model

- Kompaktes, quantisiertes Modell im GGUF-Format
- Z.B. Gemma-2-2B, Phi-3-mini, oder Qwen-2.5
- OpenAI-kompatible Chat Completions API

## Verwendung

Der Service stellt einen `/v1/chat/completions` Endpoint bereit, der mit der OpenAI Chat API kompatibel ist.

### Beispiel Request

```json
{
  "model": "gemma-2-2b",
  "messages": [
    {
      "role": "system",
      "content": "Du bist ein hilfreicher Assistent."
    },
    {
      "role": "user",
      "content": "Was ist RAG?"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 500
}
```

### Beispiel Response

```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "gemma-2-2b",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "RAG steht für Retrieval-Augmented Generation..."
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

## Modell-Downloads

GGUF-Modelle können von HuggingFace heruntergeladen werden:

```bash
# Beispiel: Gemma-2-2B quantisiert
wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf -O models/gemma-2-2b-it-Q4_K_M.gguf
```
