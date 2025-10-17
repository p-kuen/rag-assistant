# Deployment-Anleitung

## Deployment-Optionen

### 1. Lokales Development Setup

Für lokale Entwicklung mit Hot-Reloading.

```bash
# 1. Dependencies installieren
pnpm install

# 2. Docker Services starten
pnpm docker:up

# 3. Frontend Dev-Server starten
pnpm dev:frontend

# 4. Backend Dev-Server starten (in anderem Terminal)
pnpm dev:backend
```

### 2. Docker Compose Production

Komplettes System in Docker Containern.

#### Vorbereitung

```bash
# 1. LLM Modell herunterladen
docker run -v rag-assistant_llm-models:/models alpine sh -c \
  "apk add --no-cache wget && \
   wget https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf \
   -O /models/model.gguf"

# 2. Environment konfigurieren
cp docker/.env.example docker/.env
nano docker/.env  # API Keys setzen
```

#### Build & Deploy

```bash
# Production Build
docker-compose -f docker/docker-compose.yml build

# Services starten
docker-compose -f docker/docker-compose.yml up -d

# Status prüfen
docker-compose -f docker/docker-compose.yml ps

# Logs verfolgen
docker-compose -f docker/docker-compose.yml logs -f
```

#### Zugriff

- **Frontend:** http://localhost:5173
- **Backend API:** http://localhost:8080

### 3. Kubernetes Deployment

Für Production-Umgebungen mit Orchestrierung.

#### Voraussetzungen

- Kubernetes Cluster (v1.25+)
- kubectl konfiguriert
- Helm 3 (optional, aber empfohlen)

#### Namespace erstellen

```bash
kubectl create namespace rag-assistant
```

#### ConfigMap & Secrets

```yaml
# config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: rag-config
  namespace: rag-assistant
data:
  RUST_LOG: "info"
  MEILISEARCH_URL: "http://meilisearch:7700"
  EMBEDDING_API_URL: "http://embedding-api:8080"
  LLM_API_URL: "http://llm-inference:8080"
  EMBEDDING_MODEL: "BAAI/bge-small-en-v1.5"
---
apiVersion: v1
kind: Secret
metadata:
  name: rag-secrets
  namespace: rag-assistant
type: Opaque
stringData:
  MEILISEARCH_API_KEY: "your-secure-api-key-here"
```

```bash
kubectl apply -f config.yaml
```

#### PersistentVolumeClaims

```yaml
# pvc.yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: meilisearch-data
  namespace: rag-assistant
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: llm-models
  namespace: rag-assistant
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 5Gi
```

```bash
kubectl apply -f pvc.yaml
```

#### Services Deployment

```yaml
# deployments.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: meilisearch
  namespace: rag-assistant
spec:
  replicas: 1
  selector:
    matchLabels:
      app: meilisearch
  template:
    metadata:
      labels:
        app: meilisearch
    spec:
      containers:
      - name: meilisearch
        image: getmeili/meilisearch:v1.7
        ports:
        - containerPort: 7700
        env:
        - name: MEILI_ENV
          value: "production"
        - name: MEILI_MASTER_KEY
          valueFrom:
            secretKeyRef:
              name: rag-secrets
              key: MEILISEARCH_API_KEY
        - name: MEILI_NO_ANALYTICS
          value: "true"
        volumeMounts:
        - name: data
          mountPath: /meili_data
        resources:
          limits:
            cpu: "2"
            memory: "4Gi"
          requests:
            cpu: "500m"
            memory: "1Gi"
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: meilisearch-data
---
apiVersion: v1
kind: Service
metadata:
  name: meilisearch
  namespace: rag-assistant
spec:
  selector:
    app: meilisearch
  ports:
  - port: 7700
    targetPort: 7700
---
# Weitere Deployments für embedding-api, llm-inference, backend, frontend...
```

```bash
kubectl apply -f deployments.yaml
```

#### Ingress (optional)

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: rag-ingress
  namespace: rag-assistant
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: rag.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend
            port:
              number: 5173
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: backend-orchestrator
            port:
              number: 8080
```

```bash
kubectl apply -f ingress.yaml
```

### 4. Cloud-spezifische Deployments

#### AWS (ECS/Fargate)

```bash
# ECR Repository erstellen
aws ecr create-repository --repository-name rag-frontend
aws ecr create-repository --repository-name rag-backend

# Images bauen und pushen
docker build -t rag-frontend -f docker/frontend.Dockerfile apps/frontend
docker tag rag-frontend:latest <account-id>.dkr.ecr.<region>.amazonaws.com/rag-frontend:latest
docker push <account-id>.dkr.ecr.<region>.amazonaws.com/rag-frontend:latest

# ECS Task Definition und Service erstellen
aws ecs create-cluster --cluster-name rag-cluster
# ... (Task Definitions mit Fargate)
```

#### Google Cloud (Cloud Run)

```bash
# Images zu Google Container Registry pushen
gcloud builds submit --tag gcr.io/<project-id>/rag-frontend apps/frontend

# Cloud Run Service deployen
gcloud run deploy rag-frontend \
  --image gcr.io/<project-id>/rag-frontend \
  --platform managed \
  --region europe-west1 \
  --allow-unauthenticated
```

## Performance-Tuning

### Backend Orchestrator

**Rust Release Build:**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

**Environment Variables:**
```bash
# Tokio Runtime Tuning
TOKIO_WORKER_THREADS=4

# Connection Pool Sizes
MAX_HTTP_CONNECTIONS=100
```

### Meilisearch

**Indexierung-Performance:**
```bash
# Mehr RAM für Indexierung
MEILI_MAX_INDEXING_MEMORY=2GB

# Indexing Threads
MEILI_MAX_INDEXING_THREADS=4
```

**Query-Performance:**
```bash
# Search Cache aktivieren
MEILI_ENABLE_SEARCH_CACHE=true
```

### LLM Inference

**llama.cpp Optimierungen:**
```bash
# CPU-Optimierung
N_THREADS=8              # Anzahl CPU Threads
N_BATCH=512              # Batch Size für Prompt Processing

# Context Management
N_CTX=4096               # Max Context Tokens
N_KEEP=1024              # Tokens die immer im Context bleiben

# GPU (falls verfügbar)
N_GPU_LAYERS=32          # Anzahl Layer auf GPU
```

**Model Quantization:**
- Q4_K_M: Gute Balance zwischen Größe und Qualität
- Q5_K_M: Bessere Qualität, größer
- Q3_K_M: Kleiner, niedrigere Qualität

### Embedding API

**TEI Optimierungen:**
```bash
# Batch Processing
MAX_BATCH_TOKENS=16384
MAX_CLIENT_BATCH_SIZE=32

# Model Cache
HF_HOME=/models/.cache
```

## Monitoring & Logging

### Docker Logs

```bash
# Alle Logs
docker-compose -f docker/docker-compose.yml logs -f

# Spezifischer Service
docker-compose -f docker/docker-compose.yml logs -f backend-orchestrator

# Letzte N Zeilen
docker-compose -f docker/docker-compose.yml logs --tail=100 meilisearch
```

### Health Checks

```bash
# Backend Health
curl http://localhost:8080/health

# Meilisearch Health (von innerhalb des Containers)
docker exec rag-meilisearch curl http://localhost:7700/health

# LLM Health
docker exec rag-llm-inference curl http://localhost:8080/health
```

### Resource Monitoring

```bash
# Container Stats
docker stats

# Spezifische Container
docker stats rag-backend-orchestrator rag-meilisearch
```

## Backup & Recovery

### Meilisearch Backup

```bash
# Snapshot erstellen
curl -X POST 'http://localhost:7700/snapshots' \
  -H 'Authorization: Bearer YOUR_MASTER_KEY'

# Snapshot exportieren
docker cp rag-meilisearch:/meili_data/snapshots ./backups/

# Restore
docker cp ./backups/snapshot.snapshot rag-meilisearch:/meili_data/snapshots/
# Container mit --import-snapshot Flag starten
```

### LLM Models Backup

```bash
# Models sichern
docker run --rm -v rag-assistant_llm-models:/models -v $(pwd):/backup \
  alpine tar czf /backup/llm-models.tar.gz -C /models .

# Restore
docker run --rm -v rag-assistant_llm-models:/models -v $(pwd):/backup \
  alpine tar xzf /backup/llm-models.tar.gz -C /models
```

## Troubleshooting

### Service startet nicht

```bash
# Logs checken
docker-compose -f docker/docker-compose.yml logs <service-name>

# Container Status
docker-compose -f docker/docker-compose.yml ps

# Container neu starten
docker-compose -f docker/docker-compose.yml restart <service-name>
```

### Out of Memory

```bash
# Memory-Limits erhöhen in docker-compose.yml
deploy:
  resources:
    limits:
      memory: 8G  # Von 4G erhöhen
```

### Langsame Responses

1. **LLM:** Kleineres Modell verwenden (z.B. Q3 statt Q4)
2. **Embeddings:** Batch-Size erhöhen
3. **Meilisearch:** Cache aktivieren
4. **Backend:** Connection Pool erhöhen

### Network Errors

```bash
# Network prüfen
docker network inspect rag-net

# Service Connectivity testen
docker exec rag-backend-orchestrator ping meilisearch
docker exec rag-backend-orchestrator curl http://meilisearch:7700/health
```

## Security Checklist

- [ ] Meilisearch Master Key gesetzt und sicher gespeichert
- [ ] Interne Services nicht extern exponiert
- [ ] TLS/HTTPS für Production
- [ ] API Rate Limiting implementiert
- [ ] Docker Images regelmäßig updaten
- [ ] Secrets nicht in Git committen
- [ ] Firewall-Regeln für Production
- [ ] Monitoring & Alerting eingerichtet

## Updates & Upgrades

### Minor Updates

```bash
# Images pullen
docker-compose -f docker/docker-compose.yml pull

# Services neu starten
docker-compose -f docker/docker-compose.yml up -d
```

### Major Updates

1. Backup erstellen
2. Release Notes lesen
3. Test in Staging-Umgebung
4. Rolling Update in Production
5. Health Checks überwachen

## Support

Bei Problemen:
1. Logs prüfen (`docker-compose logs`)
2. Health Checks ausführen
3. Resource Usage checken (`docker stats`)
4. GitHub Issues konsultieren
