# LLM Inference Service - llama.cpp Server
# Optimized for resource efficiency and maximum performance
FROM ubuntu:22.04

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV LLAMA_CUBLAS=1
ENV LLAMA_CUDA=1

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    wget \
    curl \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Clone and build llama.cpp
RUN git clone https://github.com/ggml-org/llama.cpp.git && \
    cd llama.cpp && \
    make -j$(nproc) server

# Create models directory
RUN mkdir -p /app/models

# Copy model download script
COPY services/llm-inference/download_model.sh /app/
RUN chmod +x /app/download_model.sh

# Copy server configuration
COPY services/llm-inference/server_config.sh /app/
RUN chmod +x /app/server_config.sh

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["/app/server_config.sh"]
