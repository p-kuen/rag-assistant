#!/bin/sh

# Start the Ollama server
ollama serve &

# Pull the Mistral model
ollama pull mistral

ollama run mistral
