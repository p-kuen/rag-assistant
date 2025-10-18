#!/bin/bash

# Meilisearch Initialization Script for RAG System
# This script sets up Meilisearch with the proper configuration for the RAG system

set -e

# Configuration
MEILISEARCH_URL="http://localhost:7700"
INDEX_NAME="rag_documents"
PRIMARY_KEY="id"
WAIT_TIME=30

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Wait for Meilisearch to be ready
wait_for_meilisearch() {
    log_info "Waiting for Meilisearch to be ready..."
    
    local attempts=0
    local max_attempts=20
    
    while [ $attempts -lt $max_attempts ]; do
        if curl -s "$MEILISEARCH_URL/health" > /dev/null 2>&1; then
            log_success "Meilisearch is ready!"
            return 0
        fi
        
        attempts=$((attempts + 1))
        log_info "Attempt $attempts/$max_attempts - waiting 5 seconds..."
        sleep 5
    done
    
    log_error "Meilisearch failed to start within $((max_attempts * 5)) seconds"
    return 1
}

# Check if index exists
index_exists() {
    local response=$(curl -s "$MEILISEARCH_URL/indexes/$INDEX_NAME")
    if echo "$response" | grep -q '"uid"'; then
        return 0
    else
        return 1
    fi
}

# Create the RAG documents index
create_index() {
    log_info "Creating index: $INDEX_NAME"
    
    local response=$(curl -s -X POST "$MEILISEARCH_URL/indexes" \
        -H "Content-Type: application/json" \
        -d "{
            \"uid\": \"$INDEX_NAME\",
            \"primaryKey\": \"$PRIMARY_KEY\"
        }")
    
    if echo "$response" | grep -q '"uid"'; then
        log_success "Index '$INDEX_NAME' created successfully"
    else
        log_warning "Index creation response: $response"
        if echo "$response" | grep -q "already exists"; then
            log_info "Index '$INDEX_NAME' already exists"
        else
            log_error "Failed to create index: $response"
            return 1
        fi
    fi
}

# Configure index settings for RAG
configure_index() {
    log_info "Configuring index settings for RAG system..."
    
    local settings='{
        "searchableAttributes": [
            "content",
            "title", 
            "metadata",
            "hierarchy_lvl1",
            "hierarchy_lvl2",
            "hierarchy_lvl3"
        ],
        "filterableAttributes": [
            "document_type",
            "author",
            "created_at",
            "updated_at",
            "tags",
            "source_file",
            "chunk_index",
            "hierarchy_lvl1",
            "hierarchy_lvl2",
            "hierarchy_lvl3"
        ],
        "sortableAttributes": [
            "created_at",
            "updated_at",
            "relevance_score",
            "chunk_index"
        ],
        "rankingRules": [
            "words",
            "typo", 
            "proximity",
            "attribute",
            "sort",
            "exactness"
        ],
        "embedders": {
            "default": {
                "source": "http",
                "url": "http://embedding-api:8080/embeddings",
                "apiKey": "",
                "model": "embeddinggemma-300m",
                "dimensions": 768,
                "documentTemplate": "{{doc.hierarchy_lvl1}} {{doc.hierarchy_lvl2}} {{doc.hierarchy_lvl3}} {{doc.content}}"
            }
        }
    }'
    
    local response=$(curl -s -X PATCH "$MEILISEARCH_URL/indexes/$INDEX_NAME/settings" \
        -H "Content-Type: application/json" \
        -d "$settings")
    
    if echo "$response" | grep -q '"taskUid"'; then
        log_success "Index settings configured successfully"
        
        # Wait for settings to be applied
        log_info "Waiting for settings to be applied..."
        sleep 5
        
        # Check task status
        local task_uid=$(echo "$response" | grep -o '"taskUid":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$task_uid" ]; then
            log_info "Checking task status: $task_uid"
            local task_response=$(curl -s "$MEILISEARCH_URL/tasks/$task_uid")
            log_info "Task response: $task_response"
        fi
    else
        log_error "Failed to configure index settings: $response"
        return 1
    fi
}

# Test the index configuration
test_index() {
    log_info "Testing index configuration..."
    
    # Test search
    local search_response=$(curl -s -X POST "$MEILISEARCH_URL/indexes/$INDEX_NAME/search" \
        -H "Content-Type: application/json" \
        -d '{"q": "test", "limit": 1}')
    
    if echo "$search_response" | grep -q '"hits"'; then
        log_success "Search functionality working"
    else
        log_warning "Search test response: $search_response"
    fi
    
    # Test hybrid search
    local hybrid_response=$(curl -s -X POST "$MEILISEARCH_URL/indexes/$INDEX_NAME/search" \
        -H "Content-Type: application/json" \
        -d '{
            "q": "test",
            "hybrid": {
                "semanticRatio": 0.5,
                "embedder": "default"
            },
            "limit": 1
        }')
    
    if echo "$hybrid_response" | grep -q '"hits"'; then
        log_success "Hybrid search functionality working"
    else
        log_warning "Hybrid search test response: $hybrid_response"
    fi
}

# Show index information
show_index_info() {
    log_info "Index information:"
    
    # Get index stats
    local stats=$(curl -s "$MEILISEARCH_URL/indexes/$INDEX_NAME/stats")
    echo "$stats" | jq . 2>/dev/null || echo "$stats"
    
    echo ""
    
    # Get index settings
    local settings=$(curl -s "$MEILISEARCH_URL/indexes/$INDEX_NAME/settings")
    echo "Index settings:"
    echo "$settings" | jq . 2>/dev/null || echo "$settings"
}

# Main execution
main() {
    log_info "Starting Meilisearch initialization for RAG system..."
    
    # Wait for Meilisearch to be ready
    if ! wait_for_meilisearch; then
        log_error "Meilisearch is not available. Please start the service first."
        exit 1
    fi
    
    # Create index if it doesn't exist
    if ! index_exists; then
        create_index
    else
        log_info "Index '$INDEX_NAME' already exists"
    fi
    
    # Configure index settings
    configure_index
    
    # Test the configuration
    test_index
    
    # Show final information
    show_index_info
    
    log_success "Meilisearch initialization complete!"
    log_info "Index '$INDEX_NAME' is ready for RAG operations"
    log_info "Access Meilisearch dashboard at: $MEILISEARCH_URL"
}

# Handle script arguments
case "${1:-}" in
    "test")
        test_index
        ;;
    "info")
        show_index_info
        ;;
    "reset")
        log_warning "Resetting index '$INDEX_NAME'..."
        curl -X DELETE "$MEILISEARCH_URL/indexes/$INDEX_NAME"
        main
        ;;
    *)
        main
        ;;
esac
