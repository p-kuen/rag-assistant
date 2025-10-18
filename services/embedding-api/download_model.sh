#!/bin/bash

# Download EmbeddingGemma Model for Local Use
# This script downloads the EmbeddingGemma model for local embedding generation

set -e

# Configuration
MODEL_NAME="google/embeddinggemma-300m"
MODEL_DIR="./models"
HUGGINGFACE_CACHE_DIR="${HOME}/.cache/huggingface"

echo "🚀 Starting EmbeddingGemma model download..."

# Create models directory if it doesn't exist
mkdir -p "${MODEL_DIR}"

# Check if model is already downloaded
if [ -d "${MODEL_DIR}/${MODEL_NAME##*/}" ]; then
    echo "✅ Model already exists at ${MODEL_DIR}/${MODEL_NAME##*/}"
    echo "   Skipping download..."
    exit 0
fi

# Check if huggingface-hub is installed
if ! command -v huggingface-cli &> /dev/null; then
    echo "📦 Installing huggingface-hub..."
    pip install huggingface-hub
fi

echo "📥 Downloading ${MODEL_NAME}..."
echo "   This may take several minutes depending on your internet connection..."

# Download the model using huggingface-hub
huggingface-cli download \
    "${MODEL_NAME}" \
    --local-dir "${MODEL_DIR}/${MODEL_NAME##*/}" \
    --local-dir-use-symlinks False

echo "✅ Model downloaded successfully!"
echo "   Location: ${MODEL_DIR}/${MODEL_NAME##*/}"

# Display model information
echo ""
echo "📊 Model Information:"
echo "   - Model: ${MODEL_NAME}"
echo "   - Size: $(du -sh "${MODEL_DIR}/${MODEL_NAME##*/}" | cut -f1)"
echo "   - Files:"
ls -la "${MODEL_DIR}/${MODEL_NAME##*/}"

echo ""
echo "🎯 Next steps:"
echo "   1. Copy config.env.example to config.env"
echo "   2. Update MODEL_PATH in config.env to point to: ${MODEL_DIR}/${MODEL_NAME##*/}"
echo "   3. Run 'make build' to build the Docker image"
echo "   4. Run 'make up' to start the embedding service"
