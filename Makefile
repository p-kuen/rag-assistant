.PHONY: help install dev build docker-up docker-down docker-logs clean test

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

help: ## Zeigt diese Hilfe an
	@echo "$(BLUE)RAG Assistant - Verfügbare Kommandos:$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

install: ## Installiert alle Dependencies
	@echo "$(BLUE)Installiere Dependencies...$(NC)"
	pnpm install

dev: ## Startet Frontend und Backend im Dev-Modus
	@echo "$(BLUE)Starte Development-Server...$(NC)"
	pnpm dev

dev-frontend: ## Startet nur das Frontend
	@echo "$(BLUE)Starte Frontend...$(NC)"
	pnpm dev:frontend

dev-backend: ## Startet nur das Backend
	@echo "$(BLUE)Starte Backend...$(NC)"
	pnpm dev:backend

build: ## Baut alle Apps für Production
	@echo "$(BLUE)Baue alle Apps...$(NC)"
	pnpm build

docker-up: ## Startet alle Docker Services
	@echo "$(BLUE)Starte Docker Services...$(NC)"
	docker-compose -f docker/docker-compose.yml up -d
	@echo "$(GREEN)Services gestartet!$(NC)"
	@echo "Frontend: http://localhost:5173"
	@echo "Backend:  http://localhost:8080"

docker-down: ## Stoppt alle Docker Services
	@echo "$(YELLOW)Stoppe Docker Services...$(NC)"
	docker-compose -f docker/docker-compose.yml down

docker-logs: ## Zeigt Docker Logs
	docker-compose -f docker/docker-compose.yml logs -f

docker-ps: ## Zeigt Status der Docker Services
	docker-compose -f docker/docker-compose.yml ps

docker-restart: docker-down docker-up ## Startet Docker Services neu

docker-clean: ## Stoppt Services und löscht Volumes
	@echo "$(YELLOW)Warnung: Löscht alle Daten!$(NC)"
	@read -p "Fortfahren? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker-compose -f docker/docker-compose.yml down -v; \
		echo "$(GREEN)Volumes gelöscht$(NC)"; \
	fi

download-model: ## Lädt das default LLM Modell herunter
	@echo "$(BLUE)Lade Gemma-2-2B Modell herunter (~1.5GB)...$(NC)"
	docker run --rm -v rag-assistant_llm-models:/models alpine sh -c \
		"apk add --no-cache wget && \
		wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf \
		-O /models/model.gguf"
	@echo "$(GREEN)Modell heruntergeladen!$(NC)"

setup: install download-model docker-up ## Komplettes Setup (install + model + docker)
	@echo "$(GREEN)Setup abgeschlossen!$(NC)"

clean: ## Löscht Build-Artefakte
	@echo "$(YELLOW)Lösche Build-Artefakte...$(NC)"
	rm -rf apps/frontend/dist
	rm -rf apps/backend-orchestrator/target
	rm -rf node_modules
	rm -rf apps/*/node_modules
	@echo "$(GREEN)Clean abgeschlossen$(NC)"

test: ## Führt alle Tests aus
	@echo "$(BLUE)Führe Tests aus...$(NC)"
	pnpm test

lint: ## Führt Linting aus
	@echo "$(BLUE)Führe Linting aus...$(NC)"
	cd apps/frontend && pnpm lint
	cd apps/backend-orchestrator && cargo clippy

format: ## Formatiert den Code
	@echo "$(BLUE)Formatiere Code...$(NC)"
	cd apps/frontend && pnpm format
	cd apps/backend-orchestrator && cargo fmt

health: ## Prüft Health-Status aller Services
	@echo "$(BLUE)Prüfe Service Health...$(NC)"
	@echo -n "Backend:      "; curl -sf http://localhost:8080/health && echo "$(GREEN)✓$(NC)" || echo "$(YELLOW)✗$(NC)"
	@echo -n "Frontend:     "; curl -sf http://localhost:5173 > /dev/null && echo "$(GREEN)✓$(NC)" || echo "$(YELLOW)✗$(NC)"

backup-data: ## Erstellt Backup der Meilisearch-Daten
	@echo "$(BLUE)Erstelle Backup...$(NC)"
	mkdir -p backups
	docker run --rm -v rag-assistant_meilisearch-data:/data -v $$(pwd)/backups:/backup alpine \
		tar czf /backup/meilisearch-backup-$$(date +%Y%m%d-%H%M%S).tar.gz -C /data .
	@echo "$(GREEN)Backup erstellt in backups/$(NC)"

restore-data: ## Stellt Meilisearch-Daten wieder her (Pfad angeben: make restore-data FILE=backup.tar.gz)
	@if [ -z "$(FILE)" ]; then \
		echo "$(YELLOW)Bitte Backup-Datei angeben: make restore-data FILE=backup.tar.gz$(NC)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Stelle Backup wieder her...$(NC)"
	docker run --rm -v rag-assistant_meilisearch-data:/data -v $$(pwd)/backups:/backup alpine \
		tar xzf /backup/$(FILE) -C /data
	@echo "$(GREEN)Backup wiederhergestellt$(NC)"

stats: ## Zeigt Docker Resource-Nutzung
	docker stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}" \
		$$(docker-compose -f docker/docker-compose.yml ps -q)

.DEFAULT_GOAL := help
