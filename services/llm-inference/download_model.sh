#!/bin/bash

# Model download script for LLM Inference Service
# Downloads a compact, quantized model optimized for resource efficiency

set -e

MODEL_DIR="/app/models"
MODEL_NAME="gemma-2-2b-it-Q4_K_M"
MODEL_FILE="${MODEL_NAME}.gguf"
MODEL_URL="https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/${MODEL_FILE}"

echo "🚀 Downloading LLM model: ${MODEL_NAME}"

# Create models directory if it doesn't exist
mkdir -p "${MODEL_DIR}"

# Check if model already exists
if [ -f "${MODEL_DIR}/${MODEL_FILE}" ]; then
    echo "✅ Model already exists: ${MODEL_FILE}"
    exit 0
fi

# Download model
echo "📥 Downloading from: ${MODEL_URL}"
cd "${MODEL_DIR}"
wget -O "${MODEL_FILE}" "${MODEL_URL}"

# Verify download
if [ -f "${MODEL_FILE}" ]; then
    echo "✅ Model downloaded successfully: ${MODEL_FILE}"
    echo "📊 Model size: $(du -h ${MODEL_FILE} | cut -f1)"
else
    echo "❌ Model download failed"
    exit 1
fi

echo "🎉 Model setup complete!"
