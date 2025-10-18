#!/bin/sh

# Pull the EmbeddingGemma model
ollama pull embeddinggemma-300 && ollama serve
