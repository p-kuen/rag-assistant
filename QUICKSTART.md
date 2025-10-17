# Quick Start Guide

Schneller Einstieg in die RAG-Plattform in 5 Minuten.

## Option 1: Docker Only (Empfohlen für Demo)

### Schritt 1: LLM Modell herunterladen

```bash
# Kompaktes Modell (~1.5GB)
docker run -v rag-assistant_llm-models:/models alpine sh -c \
  "apk add --no-cache wget && \
   wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf \
   -O /models/model.gguf"
```

**Download dauert:** 5-10 Minuten bei guter Internetverbindung.

### Schritt 2: Environment konfigurieren

```bash
# .env Datei erstellen
cp docker/.env.example docker/.env

# Optional: API Key anpassen
echo "MEILISEARCH_API_KEY=$(openssl rand -base64 32)" >> docker/.env
```

### Schritt 3: Services starten

```bash
# Alle Services im Hintergrund starten
docker-compose -f docker/docker-compose.yml up -d

# Startup dauert ~2-3 Minuten beim ersten Mal
```

### Schritt 4: Status prüfen

```bash
# Alle Services sollten "Up" sein
docker-compose -f docker/docker-compose.yml ps

# Logs bei Problemen
docker-compose -f docker/docker-compose.yml logs -f
```

### Schritt 5: Zugriff

- **Frontend:** http://localhost:5173 (Production Build mit nginx)
- **Backend API:** http://localhost:8080/health

✅ **Fertig!** Die Plattform ist jetzt bereit.

**Hinweis:** Das Frontend läuft als Production Build in einem nginx Container.

---

## Option 2: Development Setup (für Entwickler)

### Schritt 1: Dependencies installieren

```bash
# Pnpm global installieren (falls noch nicht vorhanden)
npm install -g pnpm

# Projekt-Dependencies
pnpm install
```

### Schritt 2: Docker Services starten

```bash
# Nur die Backend-Services (Meilisearch, Embedding, LLM)
docker-compose -f docker/docker-compose.yml up -d meilisearch embedding-api llm-inference
```

**Hinweis:** Frontend und Backend laufen lokal im Dev-Modus.

### Schritt 3: Frontend starten

```bash
# In einem Terminal
pnpm dev:frontend

# Oder direkt
cd apps/frontend
pnpm dev
```

Frontend läuft auf: http://localhost:5173

### Schritt 4: Backend starten

```bash
# In einem anderen Terminal
pnpm dev:backend

# Oder direkt (erfordert Rust)
cd apps/backend-orchestrator
cargo run
```

Backend läuft auf: http://localhost:8080

✅ **Development-Umgebung läuft!**

---

## Erste Schritte nach dem Start

### 1. Wissensdatenbank befüllen

1. Navigiere zu **Wissensverwaltung** (`/admin/knowledge`)
2. Wähle **"Dokument Upload"** oder **"Manuelle Eingabe"**
3. Lade ein Dokument hoch oder gib Text ein
4. Warte bis Status **"succeeded"** anzeigt

### 2. Chat starten

1. Navigiere zu **Chat-Assistent** (`/chat`)
2. Stelle eine Frage zu deinem hochgeladenen Dokument
3. Erhalte eine RAG-basierte Antwort mit Quellenangaben

---

## Troubleshooting

### Services starten nicht

```bash
# Status checken
docker-compose -f docker/docker-compose.yml ps

# Logs für spezifischen Service
docker-compose -f docker/docker-compose.yml logs llm-inference

# Service neu starten
docker-compose -f docker/docker-compose.yml restart llm-inference
```

### "Model not found" Error

Das LLM-Modell wurde nicht heruntergeladen. Siehe Schritt 1.

### Port bereits belegt

Andere Anwendung nutzt Port 5173 oder 8080:

```bash
# Ports in docker-compose.yml ändern:
ports:
  - "5174:5173"  # Frontend auf 5174
  - "8081:8080"  # Backend auf 8081
```

### Out of Memory

Docker Memory-Limit erhöhen:
- **Docker Desktop:** Settings → Resources → Memory → Min. 8GB

### Frontend zeigt Fehler

1. Backend läuft nicht → `pnpm dev:backend` oder Docker-Services starten
2. CORS-Error → Backend API URL in `.env` prüfen

---

## Nächste Schritte

- 📖 Lies die [Architektur-Dokumentation](./docs/ARCHITECTURE.md)
- 🚀 Siehe [Deployment-Guide](./docs/DEPLOYMENT.md) für Production
- 🔧 Passe die [Docker-Konfiguration](./docker/docker-compose.yml) an

## Empfohlene Modelle

| Modell | Größe | RAM | Qualität | Use Case |
|--------|-------|-----|----------|----------|
| **Gemma-2-2B-IT** | ~1.5GB | 4GB | ⭐⭐⭐ | Allgemein (empfohlen) |
| Phi-3-mini | ~2.3GB | 4GB | ⭐⭐⭐⭐ | Reasoning, Coding |
| Qwen-2.5-3B | ~2.0GB | 4GB | ⭐⭐⭐⭐ | Multilingual |
| Llama-3.2-3B | ~2.0GB | 4GB | ⭐⭐⭐⭐ | Balanced |

Download-URLs findest du auf [HuggingFace](https://huggingface.co/models?library=gguf).

---

**Viel Erfolg! 🚀**
