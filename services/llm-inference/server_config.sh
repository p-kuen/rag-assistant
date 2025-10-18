#!/bin/bash

# LLM Inference Server Configuration Script
# Optimized for resource efficiency and maximum performance

set -e

MODEL_DIR="/app/models"
MODEL_NAME="gemma-2-2b-it-Q4_K_M"
MODEL_FILE="${MODEL_NAME}.gguf"
MODEL_PATH="${MODEL_DIR}/${MODEL_FILE}"

# Server configuration
HOST="0.0.0.0"
PORT="8080"
CONTEXT_SIZE="4096"
BATCH_SIZE="512"
THREADS="${LLAMA_THREADS:-$(nproc)}"
GPU_LAYERS="${LLAMA_GPU_LAYERS:-0}"

echo "🔧 Configuring LLM Inference Server..."

# Check if model exists
if [ ! -f "${MODEL_PATH}" ]; then
    echo "📥 Model not found, downloading..."
    /app/download_model.sh
fi

# Verify model exists
if [ ! -f "${MODEL_PATH}" ]; then
    echo "❌ Model file not found: ${MODEL_PATH}"
    exit 1
fi

echo "✅ Model found: ${MODEL_PATH}"
echo "📊 Model size: $(du -h ${MODEL_PATH} | cut -f1)"

# Build server command with resource optimization
SERVER_CMD="/app/llama.cpp/server"
SERVER_ARGS=(
    "--model" "${MODEL_PATH}"
    "--host" "${HOST}"
    "--port" "${PORT}"
    "--ctx-size" "${CONTEXT_SIZE}"
    "--batch-size" "${BATCH_SIZE}"
    "--threads" "${THREADS}"
    "--log-disable"
    "--metrics"
    # No API key needed for on-premise deployment
)

# Add GPU support if available
if [ "${GPU_LAYERS}" -gt 0 ]; then
    echo "🚀 GPU acceleration enabled with ${GPU_LAYERS} layers"
    SERVER_ARGS+=("--n-gpu-layers" "${GPU_LAYERS}")
else
    echo "💻 CPU-only mode with ${THREADS} threads"
fi

# Add additional optimizations
SERVER_ARGS+=(
    "--mlock"  # Lock memory to prevent swapping
    "--no-mmap"  # Disable memory mapping for better performance
    "--cont-batching"  # Enable continuous batching
)

echo "🎯 Starting llama.cpp server..."
echo "📡 Server will be available at: http://${HOST}:${PORT}"
echo "🔑 On-Premise Mode: No API key required"

# Start the server
exec "${SERVER_CMD}" "${SERVER_ARGS[@]}"
