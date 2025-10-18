#!/bin/bash

# Embedding API Server Configuration Script
# This script sets up the optimal configuration for the TEI embedding service

set -e

# Default configuration
DEFAULT_MODEL="google/embeddinggemma-300m"
DEFAULT_PORT=80
DEFAULT_MAX_BATCH_SIZE=32
DEFAULT_MAX_BATCH_TOKENS=16384

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Embedding API Server Configuration${NC}"
echo "=================================="

# Function to check if running in Docker
check_docker() {
    if [ -f /.dockerenv ]; then
        echo -e "${GREEN}✅ Running inside Docker container${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  Running on host system${NC}"
        return 1
    fi
}

# Function to detect available hardware
detect_hardware() {
    echo -e "${BLUE}🔍 Detecting hardware capabilities...${NC}"
    
    # Check for GPU
    if command -v nvidia-smi &> /dev/null; then
        echo -e "${GREEN}✅ NVIDIA GPU detected${NC}"
        GPU_COUNT=$(nvidia-smi --list-gpus | wc -l)
        echo "   GPU Count: ${GPU_COUNT}"
        export CUDA_VISIBLE_DEVICES=0
        export TORCH_DEVICE=cuda
        return 0
    elif command -v rocm-smi &> /dev/null; then
        echo -e "${GREEN}✅ AMD GPU detected${NC}"
        export CUDA_VISIBLE_DEVICES=0
        export TORCH_DEVICE=rocm
        return 0
    else
        echo -e "${YELLOW}⚠️  No GPU detected, using CPU${NC}"
        export TORCH_DEVICE=cpu
        return 1
    fi
}

# Function to optimize for CPU
optimize_cpu() {
    echo -e "${BLUE}⚙️  Optimizing for CPU inference...${NC}"
    
    # Get CPU count
    CPU_COUNT=$(nproc)
    echo "   CPU Cores: ${CPU_COUNT}"
    
    # Set optimal batch sizes for CPU
    export MAX_BATCH_SIZE=16
    export MAX_BATCH_TOKENS=8192
    export MAX_CLIENT_BATCH_SIZE=8
    export MAX_CLIENT_BATCH_TOKENS=4096
    export NUM_WORKERS=1
    
    echo "   Optimized batch sizes for CPU inference"
}

# Function to optimize for GPU
optimize_gpu() {
    echo -e "${BLUE}⚙️  Optimizing for GPU inference...${NC}"
    
    # Set optimal batch sizes for GPU
    export MAX_BATCH_SIZE=32
    export MAX_BATCH_TOKENS=16384
    export MAX_CLIENT_BATCH_SIZE=16
    export MAX_CLIENT_BATCH_TOKENS=8192
    export NUM_WORKERS=1
    
    echo "   Optimized batch sizes for GPU inference"
}

# Function to set up model configuration
setup_model() {
    echo -e "${BLUE}📦 Setting up model configuration...${NC}"
    
    # Check if local model path exists
    if [ -n "${MODEL_PATH}" ] && [ -d "${MODEL_PATH}" ]; then
        echo -e "${GREEN}✅ Using local model: ${MODEL_PATH}${NC}"
        export MODEL_ID="${MODEL_PATH}"
    else
        echo -e "${GREEN}✅ Using HuggingFace model: ${DEFAULT_MODEL}${NC}"
        export MODEL_ID="${DEFAULT_MODEL}"
    fi
    
    # Set embedding configuration
    export POOLING_MODE=mean
    export NORMALIZE_EMBEDDINGS=true
    export MAX_INPUT_LENGTH=512
}

# Function to validate configuration
validate_config() {
    echo -e "${BLUE}✅ Validating configuration...${NC}"
    
    # Check required environment variables
    required_vars=("MODEL_ID" "MAX_BATCH_SIZE" "MAX_BATCH_TOKENS")
    
    for var in "${required_vars[@]}"; do
        if [ -z "${!var}" ]; then
            echo -e "${RED}❌ Required environment variable ${var} is not set${NC}"
            exit 1
        fi
    done
    
    echo -e "${GREEN}✅ Configuration validation passed${NC}"
}

# Function to display final configuration
display_config() {
    echo ""
    echo -e "${BLUE}📋 Final Configuration:${NC}"
    echo "========================"
    echo "Model ID: ${MODEL_ID}"
    echo "Port: ${PORT:-${DEFAULT_PORT}}"
    echo "Max Batch Size: ${MAX_BATCH_SIZE}"
    echo "Max Batch Tokens: ${MAX_BATCH_TOKENS}"
    echo "Max Client Batch Size: ${MAX_CLIENT_BATCH_SIZE}"
    echo "Max Client Batch Tokens: ${MAX_CLIENT_BATCH_TOKENS}"
    echo "Pooling Mode: ${POOLING_MODE}"
    echo "Normalize Embeddings: ${NORMALIZE_EMBEDDINGS}"
    echo "Max Input Length: ${MAX_INPUT_LENGTH}"
    echo "Torch Device: ${TORCH_DEVICE:-cpu}"
    echo "Number of Workers: ${NUM_WORKERS:-1}"
    echo ""
}

# Main configuration logic
main() {
    # Detect hardware
    if detect_hardware; then
        optimize_gpu
    else
        optimize_cpu
    fi
    
    # Set up model
    setup_model
    
    # Validate configuration
    validate_config
    
    # Display final configuration
    display_config
    
    echo -e "${GREEN}🎯 Configuration complete! Ready to start embedding service.${NC}"
}

# Run main function
main "$@"
