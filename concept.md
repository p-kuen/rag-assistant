

# **Konzeptbericht: Ressourcenoptimierte Fullstack RAG-Plattform (VueJS, Rust, Docker, Meilisearch On-Premise)**

## **I. Executive Summary: Der Ressourcen-Effiziente RAG-Blueprint**

Dieser Bericht präsentiert das architektonische Konzept für eine vollständige, selbst gehostete Retrieval-Augmented Generation (RAG)-Anwendung. Das Design legt den Fokus auf maximale Kosteneffizienz und minimalen Ressourcenverbrauch durch die konsequente Nutzung von performanten, nativen Technologien und eine modulare Docker-Containerisierung. Die Notwendigkeit des On-Premise-Betriebs – bedingt durch strenge Anforderungen an Datenschutz, Datensouveränität und die Vermeidung laufender Cloud-API-Gebühren 1 – erfordert eine vollständige Dekomposition der Services, die in Cloud-Umgebungen oft als monolithische APIs bereitgestellt werden.

Das Projekt wird in fünf spezialisierte Microservices zerlegt: das **Frontend (VueJS)**, den **Backend/Orchestrator (Rust)**, die **Retrieval Engine (Meilisearch)**, den **Embedding Service** und den **LLM Inference Service**.

Die strategische Optimierung zur Erreichung der Ressourceneffizienz basiert auf drei Säulen. Erstens, der Einsatz von hochgradig optimierten Inferenz-Engines wie llama.cpp und Text Embeddings Inference (TEI) zur Gewährleistung minimaler Inferenz-Latenzen und maximalen Durchsatzes bei begrenzter lokaler Hardware.3 Zweitens, die Nutzung von **Hybrid Retrieval** in Meilisearch, welche die Stärken der Volltextsuche mit der Vektorsuche verbindet. Dies erhöht die Retrieval-Relevanz, was den Bedarf an extrem großen, rechenintensiven LLMs reduziert, da der Kontext präziser ist.5 Drittens, die Implementierung der Ingestion Pipeline in Rust zur Vermeidung von CPU-Engpässen (insbesondere des Global Interpreter Lock (GIL)), was eine schnelle und effiziente Verarbeitung großer Dokumentmengen ermöglicht.6

Die Entscheidung für einen On-Premise-Stack erfordert zwar höhere anfängliche Konfigurationsaufwände und spezialisiertes Fachwissen, ermöglicht jedoch langfristig volle Kontrolle über die Infrastruktur, gewährleistet die Datenresidenz und führt zu vorhersagbaren Betriebskosten, was für Organisationen mit hohem Datenvolumen oder strengen Compliance-Anforderungen wirtschaftlich sinnvoll ist.1

## **II. Architektur und Service-Dekomposition: Das Fünf-Service-Modell**

Die Umsetzung einer leistungsstarken, selbst gehosteten RAG-Lösung erfordert eine klare Trennung der Verantwortlichkeiten in spezialisierte Dienste. Die gewählte Architektur besteht aus fünf logischen und physisch getrennten Komponenten, die über Docker-Container orchestriert werden.

### **II.A. Die Fünf Hauptkomponenten des Stacks**

1. **Frontend (VueJS):** Die Benutzeroberfläche, die sowohl die interaktive Chat-Kommunikation mit der RAG-Logik ermöglicht als auch die administrative Oberfläche zur Befüllung des Suchindexes bereitstellt. VueJS wird aufgrund seiner Reaktivität für dynamische, flüssige Benutzeroberflächen, insbesondere für das Handling von Streaming-Antworten, ausgewählt.7  
2. **Backend/Orchestrator (Rust):** Das Herzstück der Anwendung. Dieser Dienst exponiert eine REST-API für das Frontend und implementiert die gesamte RAG-Logik. Er ist verantwortlich für das Management des Ingestion-Workflows (Parsing, Chunking, Vektorisierung) und die Orchestrierung der Echtzeit-Inferenz (Retrieval-Anfrage an Meilisearch, Context-Zusammenstellung, Generierungsanfrage an das LLM).  
3. **Retrieval Engine (Meilisearch):** Die selbst gehostete, quelloffene Suchmaschine. Meilisearch dient als der zentrale Speicherort für die indizierten Dokument-Chunks und deren assoziierte Vektor-Embeddings.8 Es liefert die essenzielle Hybrid-Suchfunktionalität, die eine Kombination aus Keyword-Matching und semantischer Suche ermöglicht.5  
4. **Embedding Service (TEI/EmbeddingGemma):** Ein dedizierter Service, der hochperformant Text-Eingaben in Vektor-Embeddings umwandelt. Diese Komponente ist kritisch, da sie während der Ingestion Phase die Vektordaten für Meilisearch erzeugt. Um die Ressourceneffizienz zu gewährleisten, wird hier ein leichtgewichtiges Open-Source-Modell verwendet.10  
5. **LLM Inference Service (llama.cpp):** Der Generierungsdienst, der das eigentliche Large Language Model (LLM) beherbergt. Er nimmt den durch RAG angereicherten Kontext entgegen und generiert die finale, menschenähnliche Antwort, die dann zurück an das Backend gestreamt wird.11

### **II.B. Datenfluss-Analyse: Ingestion vs. Retrieval**

Die Systemarchitektur ist entlang von zwei Haupt-Datenflüssen konzipiert, die unterschiedliche Anforderungen an Latenz und Durchsatz stellen:

#### **Ingestion Phase (Offline)**

Die Ingestion ist ein asynchroner, I/O- und CPU-intensiver Prozess, der nicht in Echtzeit ausgeführt werden muss, aber extrem performant sein muss, um große Dokumentmengen schnell zu verarbeiten. Der Fluss ist: Das VueJS-Frontend sendet das Dokument (zunächst Markdown, später PDF) an das **Rust Backend**. Das Rust Backend führt das performante Parsing und Chunking durch. Die Chunks werden an den **Embedding Service** gesendet, um Vektoren zu erzeugen. Die resultierenden Chunks, Vektoren und angereicherten Metadaten werden schließlich an **Meilisearch** zur Indexierung übermittelt.12

#### **Retrieval/Inference Phase (Online/Echtzeit)**

Dieser Pfad erfordert niedrige Latenz, da er die direkte Benutzerinteraktion darstellt. Die Benutzeranfrage geht vom VueJS-Frontend an das **Rust Backend**. Das Backend orchestriert den RAG-Workflow: Es sendet eine Hybrid-Suchanfrage an **Meilisearch**.5 Die relevanten Dokument-Chunks werden abgerufen, mit einem System-Prompt kombiniert und an den **LLM Inference Service** gesendet. Der LLM Service generiert die Antwort und streamt diese (SSE) zurück über das Rust Backend an das VueJS Frontend.7

### **II.C. Justification für Rust und VueJS im RAG-Kontext**

Die Wahl von Rust als Backend-Sprache ist eine bewusste Entscheidung zugunsten von Performance und Ressourceneffizienz. Rust bietet garantierte Speichersicherheit und Zero-Cost Abstraktionen, was es ideal für die rechenintensiven Teile der Ingestion-Pipeline (Parsing, Chunking) macht.6 Darüber hinaus ist die Nutzung der asynchronen Laufzeitumgebung tokio für I/O-gebundene Aufgaben – wie das Warten auf Antworten von Meilisearch, dem Embedder oder dem LLM – entscheidend, um die Latenz optimal zu überbrücken und den Durchsatz zu maximieren.

Ein wesentliches architektonisches Muster, das sich aus der On-Premise-Anforderung ergibt, ist die Notwendigkeit, einen lokalen Ersatz für proprietäre Cloud-APIs zu schaffen. Sowohl der gewählte Embedding Service (TEI) als auch der LLM Inference Service (llama.cpp Server) unterstützen das **OpenAI Embeddings/Chat Completions API-Protokoll**.4 Dies ist ein signifikanter Vorteil, da der Rust Backend Orchestrator einen einzigen, robusten asynchronen Client, wie etwa openai4rs 14, verwenden kann, um beide lokalen Dienste anzusprechen. Diese Standardisierung vereinfacht die Implementierung der RAG-Logik erheblich und schützt das Backend vor der proprietären Komplexität der zugrundeliegenden Inferenz-Engines.

## **III. Core Layer 1: Container Orchestration (Docker Compose)**

Der gesamte Stack wird durch Docker Compose orchestriert, wodurch eine leicht reproduzierbare und verwaltbare On-Premise-Umgebung geschaffen wird.15 Die Konfiguration muss nicht nur die Dienste definieren, sondern auch explizit Vorkehrungen für Ressourcenbegrenzung und Persistenz treffen.

### **III.A. Struktur der Docker Compose Datei**

Alle fünf Dienste (frontend, backend-rust, meilisearch, embedding-api, llm-api) werden in einer gemeinsamen docker-compose.yml definiert und einem internen Docker-Netzwerk (rag-net) zugewiesen. Dies ermöglicht eine schnelle, latenzarme Kommunikation zwischen den Diensten, was für den synchronen RAG-Workflow (BE ruft Meilisearch ruft LLM) kritisch ist.

Die On-Premise-Natur erfordert die Definition von Persistent Volumes, um sicherzustellen, dass die Meilisearch-Indexdatenbank und die LLM-Modelldateien (.gguf) lokal gespeichert werden und einen Neustart des Containers oder Hosts überdauern. Dies gewährleistet die Datenhoheit und die Vermeidung von erneuten Ingestion-Prozessen.

### **III.B. Ressourcenschonende Konfiguration und Docker Limits**

Die größte Herausforderung beim lokalen Hosting liegt in der Verwaltung der Systemressourcen. Da Container standardmäßig keine Beschränkungen haben und theoretisch die gesamte Host-CPU und den RAM nutzen können 16, besteht die Gefahr eines Out-of-Memory Exception (OOME) des Host-Betriebssystems.

Um die Ressourceneffizienz zu gewährleisten und das Host-System zu schützen, müssen explizite Ressourcenlimits über deploy.resources.limits gesetzt werden. Dies gilt insbesondere für die beiden rechenintensivsten Komponenten:

1. **Meilisearch:** Obwohl Meilisearch effizient ist, wächst der RAM-Bedarf proportional zur Größe des Index und der Dimensionalität der Vektoren.17 Eine Begrenzung des Speichers ist notwendig, um Stabilität zu gewährleisten.  
2. **LLM Inference API und Embedding API:** Diese Dienste, die für die Durchführung von Rechenoperationen auf großen Matrizen verantwortlich sind, sind die Hauptverbraucher von CPU und RAM. Durch das Setzen von Limits (--memory, \--cpus) wird verhindert, dass sie die Host-Maschine überwältigen, insbesondere wenn die Inferenz auf die CPU zurückfällt.16

Ein entscheidender Aspekt zur Erfüllung der Ressourceneffizienz ist die Wahl der Inferenz-Engine. Standard-RAG-Stacks nutzen häufig Ollama für die Benutzerfreundlichkeit.3 Jedoch zeigen Vergleiche, dass die native C/C++-Implementierung von **Vanilla llama.cpp** eine signifikant höhere Geschwindigkeit aufweist (Berichten zufolge bis zu 30% schneller als Ollama bei quantisierten Modellen).3 Für ein Lehrprojekt mit dem primären Ziel der maximalen Ressourcenschonung ist diese Leistungssteigerung entscheidend, da eine höhere Token/Sekunde-Rate die effektive Nutzungsdauer der Hardware verlängert und die Betriebskosten senkt. Die Expertenentscheidung lautet daher, den llama-server oder die spezialisierte TEI-Engine zu verwenden und auf den Komfort-Wrapper Ollama zu verzichten. Wenn eine lokale GPU verfügbar ist, muss die Konfiguration mittels deploy oder docker compose \--profile linux-gpu up den Zugriff auf die GPU (z.B. über CUDA) ermöglichen, um die Inferenzgeschwindigkeiten drastisch zu erhöhen.4

## **IV. Core Layer 2: Retrieval Engine (Meilisearch Vector Hybrid Strategy)**

Meilisearch dient in diesem Projekt als die zentrale Retrieval-Komponente, die weit über die Funktion einer reinen Vektordatenbank hinausgeht.

### **IV.A. Meilisearch RAG Setup und Hybrid Search**

Der Kernvorteil von Meilisearch für RAG-Anwendungen liegt in der nativen **Hybrid Search**.9 Im Gegensatz zu reinen Vektordatenbanken, die ausschließlich auf die semantische Ähnlichkeit der Vektoren setzen 20, kombiniert Meilisearch die Vektor-Suche mit robuster, fehlertoleranter Volltextsuche (Keyword-Matching).5

Diese Kombination ist architektonisch wertvoll, insbesondere im Kontext ressourcenschonender On-Premise-LLMs. Kleinere, quantisierte Modelle können Vektoren erzeugen, die weniger nuanciert sind. Die robuste Keyword-Suche fungiert als stabiles Fallback, das die Relevanz der abgerufenen Chunks auch bei suboptimalen Embeddings garantiert. Die Relevanz wird durch den einzigartigen **Bucket-Sorting-Algorithmus** von Meilisearch optimiert, der Dokumente basierend auf vordefinierten Ranking-Regeln (wie Typo-Toleranz und Wortnähe) sortiert.5

Das Ergebnis der beiden Suchmethoden (Semantik und Volltext) wird durch **Score Normalization** harmonisiert: Die Vektor-Ähnlichkeitsscores werden mittels affiner Transformation in den gleichen 0-1-Bereich wie die Volltext-Scores gebracht. Dies ermöglicht es dem Rust Backend, zur Laufzeit über einen semanticRatio (z.B. 0.5) festzulegen, wie stark die semantische Suche gewichtet werden soll.5

Die Initialisierung und Konfiguration des Index erfolgt durch das Rust Backend unter Verwendung des meilisearch-sdk 21, wobei insbesondere die embedders-Konfiguration so gesetzt wird, dass sie auf den internen Docker-Host des lokalen Embedding Service verweist.12

### **IV.B. Dokumenten-Template und Metadaten-Nutzung**

Um eine präzise Retrieval-Leistung zu erzielen, muss der Ingestion-Prozess über das einfache Speichern von Text hinausgehen.

#### **Konfiguration des Embedders**

Meilisearch ermöglicht die Konfiguration eines documentTemplate.12 Dieses Template ist ein kritischer Optimierungshebel: Es definiert, welche Teile des Dokumenten-Chunks in den Embedder eingespeist werden, um den Vektor zu generieren. Durch die Kombination von hierarchischen Metadaten (z.B. doc.hierarchy\_lvl1) mit dem eigentlichen Chunk-Inhalt (doc.content) wird sichergestellt, dass der Embedder den *Kontext* des Chunks (etwa: "Dieser Absatz stammt aus Kapitel 3.1: Technische Spezifikationen") versteht. Dies verbessert die Retrieval-Relevanz und reduziert gleichzeitig die an den Embedder gesendeten Token, was die Kosten und die Rechenlast im lokalen Embedding Service senkt.12

#### **Meilisearch als Context-Gatekeeper**

Die Rolle von Meilisearch in der RAG-Architektur geht über die reine Speicherung und Suche hinaus; sie dient als "Context-Gatekeeper". Um präzisere Suchanfragen zu ermöglichen, muss das Rust Backend während der Ingestion die Metadaten des Dokuments (Titel, Autor, Datum, Dokumenttyp) extrahieren und an Meilisearch übermitteln. Diese Metadaten müssen dann als filterableAttributes im Meilisearch-Index konfiguriert werden.12 Dadurch kann der Rust Orchestrator die Vektorsuche vorab filtern (z.B. "Suche nur in Dokumenten, die nach 2023 veröffentlicht wurden"), was die Menge der zu durchsuchenden Vektoren reduziert und die Ergebnisqualität erhöht, bevor die rechenintensivere semantische Suche beginnt.

## **V. Core Layer 3: Optimized Inference Stack (LLM und Embeddings)**

Die Einhaltung der strengen Anforderungen an Ressourceneffizienz und On-Premise-Hosting führt zu spezifischen Entscheidungen hinsichtlich der Modelle und deren Laufzeitumgebung.

### **V.A. Auswahl und Deployment des Embedding Models**

Zur Erzeugung der Vektoren wird ein hochperformantes und speichereffizientes Open-Source-Modell benötigt.

#### **Modellwahl und MRL**

Die Analyse empfiehlt **EmbeddingGemma** 10 oder nomic-embed-text.23 EmbeddingGemma sticht hervor, da es eines der leistungsstärksten mehrsprachigen Modelle unter 500 Millionen Parametern ist und quantisiert mit weniger als 200MB RAM laufen kann.10 Dies macht es ideal für den ressourcenschonenden On-Premise-Betrieb.

Ein entscheidender technischer Vorteil von EmbeddingGemma ist die Nutzung von **Matryoshka Representation Learning (MRL)**.10 MRL ermöglicht es, die standardmäßigen 768-dimensionalen Vektoren zur Laufzeit auf kleinere Dimensionen (z.B. 256 oder 512\) zu kürzen, ohne signifikanten Qualitätsverlust. Diese Reduzierung der Vektordimension hat direkte Auswirkungen auf die Kosteneffizienz des gesamten Systems, da es den Speicherbedarf von Meilisearch senkt und die Retrieval-Geschwindigkeit verbessert.10

#### **Deployment Engine: Text Embeddings Inference (TEI)**

Das Deployment erfolgt über die **Text Embeddings Inference (TEI)** Engine von HuggingFace, die speziell für die Bereitstellung von Embedding-Modellen über eine performante REST-API (OpenAI-kompatibel) optimiert wurde.4 Der TEI-Container kann entweder für CPU- oder CUDA-Betrieb konfiguriert werden, wobei das spezifische Modell (z.B. google/embeddinggemma-300m) geladen wird. Dies schafft den notwendigen, standardisierten API-Endpunkt für das Rust Backend.4

Tabelle: Embedding Model Comparative Analysis for On-Premise Deployment

| Modell | Parameteranzahl | Typische Vektor-Dimension | RAM-Bedarf (Quantisiert) | Effizienzmerkmal |
| :---- | :---- | :---- | :---- | :---- |
| EmbeddingGemma 10 | \<500M | 768 (MRL: 128-512 optional 10) | \<200 MB | Matryoshka Representation Learning (MRL) ermöglicht Dimensionstrunkierung, senkt Speicherbedarf und Latenz. |
| nomic-embed-text 23 | 137M | 768 | \~274 MB | Übertrifft proprietäre Modelle (ada-002) bei geringerer Größe und gutem Ressourcen-/Genauigkeitsverhältnis. |

### **V.B. Auswahl und Deployment des LLM Inference Servers**

Der LLM Inference Server ist die kritische Komponente für die Generierung von Antworten.

#### **Modellwahl und Inference Engine**

Es wird empfohlen, ein hoch-quantisiertes, kleines bis mittelgroßes, aktuelles LLM (wie Llama 3 8B oder Phi-3 Mini) im GGUF-Format zu verwenden. Die entscheidende Wahl der Engine ist der **Vanilla llama.cpp Server** (llama-server).11

Obwohl Ollama für Anfänger komfortabler ist 3, erzielt llama.cpp als native C/C++-Implementierung ohne Wrapper-Overhead eine bis zu 30% schnellere Token-Verarbeitungsrate.3 Im On-Premise-Betrieb, wo Rechenressourcen fix und begrenzt sind, stellt diese Leistungssteigerung einen direkten ökonomischen Vorteil dar, indem sie die Amortisationszeit der Hardware verkürzt und mehr Anfragen pro Zeiteinheit ermöglicht. Die technische Komplexität der manuellen Konfiguration wird zugunsten der langfristigen Ressourceneffizienz in Kauf genommen.2

Der llama-server bietet ebenfalls einen OpenAI-kompatiblen API-Endpunkt 11, wodurch er nahtlos in die Architektur integriert werden kann, indem er denselben openai4rs-Client im Rust Backend nutzt wie der Embedding Service.

#### **Hardware-Optimierung**

llama.cpp unterstützt nativ die Optimierung für verschiedene Architekturen, einschließlich AVX/AVX2/AVX512 für x86-CPUs und ARM NEON/Metal für Apple Silicon.11 Durch die Nutzung dieser spezifischen Hardware-Fähigkeiten wird die CPU-Inferenzleistung maximiert, was die Ressourceneffizienz des On-Premise-Stacks weiter untermauert.

Tabelle: Resource Optimization Comparative Analysis: LLM Inference

| Feature | Vanilla llama.cpp Server | Ollama Wrapper | Relevanz für On-Premise-Effizienz |
| :---- | :---- | :---- | :---- |
| Basisleistung | Hochgradig optimiertes C/C++, bis zu 30% schneller.3 | Bequemer Wrapper über llama.cpp. | Entscheidend für die Maximierung des Durchsatzes bei begrenzter lokaler Hardware.18 |
| Konfigurationskontrolle | Manuelle Parametrierung für Hardware-Abstimmung (z.B. Quantisierung). | Automatische, weniger granulare Einstellungen. | Ermöglicht die Feinabstimmung auf spezifische Hardware, was für langfristige Kostenvorhersagbarkeit unerlässlich ist.3 |
| Ressourcen-Footprint | Minimaler Festplattenspeicher und geringere Startlatenz.24 | Größerer Overhead durch die Wrapping-Schicht. | Trägt direkt zur Anforderung der Ressourcenschonung bei.24 |

## **VI. Rust Backend Services: Die High-Performance Ingestion Pipeline**

Das Rust Backend ist darauf ausgelegt, die Ingestion von Dokumenten so schnell und effizient wie möglich zu gestalten. Hier wird die Stärke von Rust in der I/O- und CPU-Intensität voll ausgeschöpft.

### **VI.A. Architektur der Ingestion API**

Der Rust-Server wird unter Verwendung eines asynchronen Frameworks wie actix-web oder axum betrieben, das auf der tokio-Laufzeitumgebung basiert. Der reqwest Crate dient als asynchroner HTTP-Client für die Kommunikation mit Meilisearch, dem Embedder und dem LLM.

### **VI.B. Dokumenten-Parsing und \-Extraktion**

Die Dokumentenverarbeitung erfordert spezialisierte, performante Crates, um den GIL-Flaschenhals zu vermeiden, der in Python-Lösungen auftritt, insbesondere bei CPU-gebundenen Aufgaben wie dem Parsen großer Dateien.6 Die Wahl rein nativer Rust-Crates ist daher eine architektonische Notwendigkeit für maximale Ressourceneffizienz.

1. **Markdown Ingestion:** Der Crate markdown-parser 25 kann Markdown-Dateien effizient analysieren, einschließlich der Extraktion von optionalem Front Matter (YAML, JSON, TOML). Dieses Front Matter ist entscheidend, da es initiale Metadaten (z.B. Autor, Kategorie) liefert, die jedem Chunk hinzugefügt werden müssen.  
2. **PDF Ingestion (Roadmap-Erweiterung):** Für die geplante Unterstützung von PDF-Dokumenten wird der Einsatz von **extractous** 6 dringend empfohlen. extractous ist eine native Rust-Lösung, die speziell für eine hohe Verarbeitungsgeschwindigkeit (bis zu 25x schneller als populäre Python-Alternativen) und geringen Speicherverbrauch bei der Extraktion von Inhalt und Metadaten aus unstrukturierten Daten entwickelt wurde.6 Alternativ bietet oxidize-pdf 27 eine ebenfalls rein native Rust-Lösung ohne externe C-Abhängigkeiten.

### **VI.C. RAG-Vorbereitungslogik (Chunking, Metadata Enrichment)**

1. **Intelligentes Chunking:** Die Qualität der Retrieval hängt stark von der Größe und Kohärenz der Chunks ab.12 Der Rust Backend sollte keine willkürlichen Zeichenlimits verwenden, sondern intelligente, semantische Chunking-Strategien implementieren. Crates wie rag-toolchain::chunkers oder llm\_utils::TextChunker 28 sind geeignet, da sie Text basierend auf Token-Größen oder semantischen Grenzen (z.B. Absätzen oder Überschriften) in *balancierte* Chunks unterteilen, was den Kontext intakt hält.12  
2. **Metadaten-Anreicherung:** Jeder generierte Chunk muss mit umfassenden Metadaten versehen werden, die während der Parsing-Phase extrahiert wurden (Dateiname, Hierarchie, ursprüngliche Metadaten). Diese angereicherten Metadaten werden an Meilisearch übergeben und dienen später sowohl der Filterung als auch der Quellenzuordnung (Source Attribution).12

### **VI.D. Vektorisierung und Indexierung**

Nach der Erzeugung der Chunks sendet das Rust Backend diese asynchron an den lokalen TEI Embedding Service (V.A.) über den OpenAI-kompatiblen API-Endpunkt. Die empfangenen Vektoren werden mit dem Chunk-Text und den Metadaten zu einem JSON-Objekt kombiniert. Schließlich wird das Array dieser fertigen Dokumente mithilfe des meilisearch-sdk 22 an den konfigurierten Meilisearch-Index gesendet.

Tabelle: RAG Ingestion Pipeline Flow (Rust Backend)

| Schritt | Rust Komponente/Crate | Funktion | Ausgabe |
| :---- | :---- | :---- | :---- |
| 1\. Dokumenten-Upload | Rust API Endpoint (z.B. actix-web) | Akzeptiert Datei (Markdown/PDF) via HTTP POST. | Binär-/Textinhalt. |
| 2\. Parsing/Extraktion | markdown-parser / extractous 6 | Extrahiert sauberen Text und hierarchische Metadaten. | Strukturierte Textblöcke und Metadaten (JSON). |
| 3\. Chunking | rag-toolchain::chunkers / llm\_utils 28 | Zerlegt Text in kontextuell tragfähige, balancierte Stücke. | Liste optimierter Text-Chunks. |
| 4\. Vektorisierung | Async HTTP Client (reqwest) | Ruft lokale Embedding API (TEI) auf (OpenAI-konform). | Chunks mit Vektoren (z.B. 768-dim oder trunkierte 512-dim 10). |
| 5\. Indexierung | meilisearch-sdk 22 | Pusht das finale JSON-Array (Chunk \+ Vektor \+ Metadaten \+ eindeutige ID) an Meilisearch.12 | Indexierte Dokumente (Meilisearch Index). |

## **VII. Rust Backend Services: Die RAG Inference API**

Der Inference-Dienst steuert den Echtzeit-Dialog des Nutzers mit dem indizierten Wissen. Er ist für die Orchestrierung der schnellen Abfolge von Retrieval und Generation verantwortlich.

### **VII.A. Der RAG-Workflow in Rust**

1. **Query Transformation (Empfohlen):** Um die Retrieval-Qualität zu maximieren, kann das Rust Backend den ursprünglichen Nutzer-Query optional an das LLM senden, um eine optimierte Suchanfrage zu generieren (z.B. Umschreibung einer umgangssprachlichen Frage in präzisere Keyword-Phrasen).5  
2. **Hybrid Retrieval (Meilisearch):** Das Backend nutzt das meilisearch-sdk 21, um die transformierte (oder Original-) Suchanfrage als Hybrid-Search-Call auszuführen. Dabei müssen die Filter-Optionen (basierend auf den in VI.C. erfassten Metadaten) und der semanticRatio konfiguriert werden, um die Top-k relevanten Chunks abzurufen.  
3. **Context Compiling:** Die abgerufenen Chunks und deren Metadaten werden in einen Kontextblock zusammengefasst. Dieser Kontext wird zusammen mit einem vordefinierten, instruierenden System-Prompt (der dem LLM Anweisungen zur Beantwortung gibt) und der ursprünglichen Benutzerfrage an den LLM Inference Server gesendet.  
4. **Generation Call (LLM Inference):** Der Rust BE sendet den kompilierten Kontext über den openai4rs Client an den lokalen llama-server. Dieser Client muss den Chat Completion Call im **Streaming-Modus** ausführen.

### **VII.B. Implementierung des Chat-Streaming und Source Attribution**

Um die Latenz der lokalen Inferenz zu kaschieren und ein modernes Benutzererlebnis zu gewährleisten, ist das Streaming der LLM-Antworten (Server-Sent Events/SSE) unerlässlich.19 Der Rust Backend muss die eingehenden Token vom llama-server unmittelbar und asynchron an das VueJS Frontend weiterleiten, ohne auf das vollständige Ende der Generierung zu warten.

Ein fortgeschrittenes RAG-System muss Vorkehrungen zur Minderung von Halluzinationen treffen, indem es die Antworten mit der Quelle belegt (Source Attribution).7 Dies erfordert, dass das Rust RAG Service eine spezielle JSON-Antwortstruktur definiert. Diese Struktur muss nicht nur den gestreamten generierten Text, sondern auch ein Array der Metadaten der verwendeten Quellen (z.B. Titel, Dateiname, Hierarchiepfad der Chunks), die Meilisearch im Retrieval-Schritt zurückgegeben hat, an das Frontend kommunizieren.12

## **VIII. VueJS Frontend Design und Interaktion**

Das VueJS Frontend dient als Zugangspunkt und muss die Komplexität des Backends in eine nutzerfreundliche und reaktive Oberfläche übersetzen.

### **VIII.A. Die Ingestion-Oberfläche**

Die Ingestion-Oberfläche muss es dem Nutzer ermöglichen, Markdown-Dateien hochzuladen.

* **Asynchrones Task-Management:** Da der Ingestion-Prozess (Parsing, Chunking, Vektorisierung, Indexierung) zeitaufwendig ist, darf das Frontend nicht blockieren. Nach dem Upload muss das Frontend eine Task ID vom Rust Backend erhalten. Anschließend muss es periodisch den Status dieses Tasks abfragen, um den Nutzer über den Fortschritt zu informieren (z.B. ob die Indexierung enqueued, processing oder succeeded ist).12  
* **Vorbereitung für die PDF-Roadmap:** Die UI sollte bereits für die spätere PDF-Unterstützung Metadaten-Eingabefelder (z.B. für tags oder document\_type) bereitstellen. Diese Metadaten werden vom Frontend gesammelt und zusammen mit der Datei an den Rust Backend gesendet, wo sie in die Ingestion Pipeline eingespeist werden (Metadaten-Anreicherung).

### **VIII.B. Die Conversational Chat-Oberfläche**

Die Chat-Oberfläche ist das zentrale Feature und muss eine reaktive User Experience bieten.

* **Streaming-Erfahrung:** Das Frontend muss Server-Sent Events (SSE) vom Rust Backend effizient verarbeiten können, um die LLM-Antwort Token für Token in Echtzeit anzuzeigen.  
* **Source Attribution Display:** Neben der generierten Antwort muss die UI die Möglichkeit bieten, die verwendeten Dokumenten-Chunks und deren Metadaten (Quellen) anzuzeigen. Dies wird in einem separaten Designelement dargestellt, um die Vertrauenswürdigkeit der KI-Antwort zu erhöhen, indem die Faktenbasis sichtbar gemacht wird.7  
* **Interaktive Elemente:** Optional kann die UI die durch den Rust Backend durchgeführte Query Transformation anzeigen, was dem Nutzer Einblick in die Funktionsweise des RAG-Systems gibt und als wertvolles Lernmittel im Rahmen des Lehrgangs dient.

## **IX. Datenverarbeitungs-Workflow (RAG Flow Diagramm)**

Der gesamte Retrieval-Augmented Generation (RAG) Workflow ist in zwei Hauptphasen unterteilt: die Ingestion (Datenvorbereitung und Indexierung) und die Inference (Echtzeit-Suche und Generierung). Beide Phasen sind modular aufgebaut und auf Ressourceneffizienz optimiert.

| Phase | Schritt | Komponente | Beschreibung |
| :---- | :---- | :---- | :---- |
| **Ingestion (Befüllung des Wissensspeichers)** | 1\. Dokumenten-Upload | Frontend (VueJS) | Der Nutzer lädt die Quelldokumente (Markdown, später PDF) über die Admin-Oberfläche hoch. |
|  | 2\. Parsing & Chunking | Backend (Rust) | Das Rust-Backend extrahiert den Text und führt semantisches Chunking durch, um kontextuell kohärente Textabschnitte zu erstellen. 6 |
|  | 3\. Metadaten-Anreicherung | Backend (Rust) | Wichtige Metadaten (Titel, Hierarchie, Tags) werden extrahiert und jedem Chunk hinzugefügt. 12 |
|  | 4\. Vektorisierung | Embedding Service (TEI) | Der dedizierte TEI-Service generiert hoch-performante Vektor-Embeddings für jeden Chunk (OpenAI-kompatible API). 4 |
|  | 5\. Indexierung | Retrieval Engine (Meilisearch) | Chunks, Vektoren und Metadaten werden im Meilisearch-Index gespeichert und die Hybrid-Suchfunktion aktiviert. 4 |
| **Inference (Echtzeit-Chat und Generierung)** | 6\. Nutzeranfrage (Query) | Frontend (VueJS) | Der Nutzer stellt eine Frage über die Chat-Oberfläche. |
|  | 7\. (Optional) Query Transformation | Backend (Rust) | Das Backend sendet die Frage an das LLM, um sie in eine optimierte, präzise Suchanfrage umzuformulieren. 5 |
|  | 8\. Hybrid Retrieval | Retrieval Engine (Meilisearch) | Die Engine führt eine kombinierte Keyword- und Vektorsuche durch, filtert nach Metadaten und liefert die Top-k relevanten Chunks zurück. 5 |
|  | 9\. Context Compiling | Backend (Rust) | Die abgerufenen Chunks werden zusammen mit einem instruierenden System-Prompt und der ursprünglichen Frage zu einem finalen Kontext kombiniert. |
|  | 10\. Response Generation (Stream) | LLM Inference Service (llama.cpp) | Das LLM generiert die Antwort und streamt die Token über den OpenAI-kompatiblen API-Endpunkt an das Rust-Backend. 11 |
|  | 11\. Antwort & Attribution | Frontend (VueJS) | Das Frontend zeigt die gestreamte Antwort des LLM in Echtzeit an und blendet die zugehörigen Quellen (Metadaten der Chunks) zur Faktencheck-Möglichkeit ein. 7 |

## **X. Fazit und Architektonische Empfehlung**

Das vorgeschlagene Fullstack RAG-Konzept erfüllt die Anforderungen an einen kosteneffizienten, ressourcenschonenden und vollständig On-Premise gehosteten Stack. Die Architektur löst die inhärenten Herausforderungen des lokalen Betriebs durch die konsequente Dekomposition und die gezielte Auswahl hochperformanter, nativer Technologien.

Die wichtigsten architektonischen Empfehlungen zur Erreichung der maximalen Effizienz sind:

1. **Performanz über Komfort:** Die Entscheidung, den Vanilla llama.cpp Server und die Text Embeddings Inference (TEI) Engine anstelle von Ollama zu verwenden, ist technisch zwingend, um die maximale Token/Sekunde-Leistung auf begrenzter On-Premise-Hardware zu erzielen (bis zu 30% schneller).3  
2. **Rust als I/O-Master:** Der Einsatz von Rust und nativen Crates wie extractous für die Dokumentenverarbeitung gewährleistet, dass die ressourcenintensivste Phase (Ingestion, Chunking) den Global Interpreter Lock (GIL) vermeidet, was eine schnelle und skalierbare Befüllung des Suchindexes ermöglicht.6  
3. **Wirtschaftliche Vektordatenbank:** Die Auswahl von EmbeddingGemma und die Nutzung der Matryoshka Representation Learning (MRL)-Funktion erlauben die Truncation der Vektordimensionen.10 Dies senkt den Speicher-Footprint von Meilisearch, verbessert die Retrieval-Geschwindigkeit und trägt direkt zur Kosteneffizienz bei, ohne die semantische Genauigkeit drastisch zu kompromittieren.  
4. **Standardisierung der Schnittstellen:** Die Verwendung des OpenAI-kompatiblen Protokolls über den gesamten lokalen Inference Stack vereinfacht die Orchestrierung im Rust Backend erheblich und ermöglicht einen einheitlichen und robusten Client-Code.4

Die Implementierung dieses Konzepts resultiert in einem robusten, performanten Prototypen, der alle Anforderungen des KI-Lehrgangs erfüllt und die tiefgreifenden technischen Trade-offs bei der Entwicklung ressourceneffizienter, privater RAG-Lösungen demonstriert.

#### **Referenzen**

1. A Cost-Benefit Analysis of On-Premise Large Language Model Deployment: Breaking Even with Commercial LLM Services \- arXiv, Zugriff am Oktober 17, 2025, [https://arxiv.org/html/2509.18101v1](https://arxiv.org/html/2509.18101v1)  
2. Running Large Language Models On-Premises: A Comprehensive Guide \- Medium, Zugriff am Oktober 17, 2025, [https://medium.com/@Mamoutou/running-large-language-models-on-premises-a-comprehensive-guide-ebb5e8a64c36](https://medium.com/@Mamoutou/running-large-language-models-on-premises-a-comprehensive-guide-ebb5e8a64c36)  
3. Help choosing between Ollama, llama.cpp, or something else for background LLM server (used with dictation) : r/LocalLLaMA \- Reddit, Zugriff am Oktober 17, 2025, [https://www.reddit.com/r/LocalLLaMA/comments/1mdma9a/help\_choosing\_between\_ollama\_llamacpp\_or/](https://www.reddit.com/r/LocalLLaMA/comments/1mdma9a/help_choosing_between_ollama_llamacpp_or/)  
4. Welcome EmbeddingGemma, Google's new efficient embedding model \- Hugging Face, Zugriff am Oktober 17, 2025, [https://huggingface.co/blog/embeddinggemma](https://huggingface.co/blog/embeddinggemma)  
5. Mastering RAG: unleashing precision and recall with Meilisearch's ..., Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/blog/mastering-rag](https://www.meilisearch.com/blog/mastering-rag)  
6. yobix-ai/extractous: Fast and efficient unstructured data extraction. Written in Rust with bindings for many languages. \- GitHub, Zugriff am Oktober 17, 2025, [https://github.com/yobix-ai/extractous](https://github.com/yobix-ai/extractous)  
7. OnPrem.LLM: A Privacy-Conscious Document Intelligence Toolkit \- arXiv, Zugriff am Oktober 17, 2025, [https://arxiv.org/html/2505.07672v3](https://arxiv.org/html/2505.07672v3)  
8. AI agents & RAG \- Meilisearch, Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/solutions/rag](https://www.meilisearch.com/solutions/rag)  
9. 10 Best RAG Tools and Platforms: Full Comparison \[2025\] \- Meilisearch, Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/blog/rag-tools](https://www.meilisearch.com/blog/rag-tools)  
10. Introducing EmbeddingGemma: The Best-in-Class Open Model for On-Device Embeddings \- Google for Developers Blog, Zugriff am Oktober 17, 2025, [https://developers.googleblog.com/en/introducing-embeddinggemma/](https://developers.googleblog.com/en/introducing-embeddinggemma/)  
11. ggml-org/llama.cpp: LLM inference in C/C++ \- GitHub, Zugriff am Oktober 17, 2025, [https://github.com/ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp)  
12. How to build a RAG system (with Meilisearch), Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/blog/how-to-build-rag](https://www.meilisearch.com/blog/how-to-build-rag)  
13. Build Advanced Retrieval-Augmented Generation Systems | Microsoft Learn, Zugriff am Oktober 17, 2025, [https://learn.microsoft.com/en-us/azure/developer/ai/advanced-retrieval-augmented-generation](https://learn.microsoft.com/en-us/azure/developer/ai/advanced-retrieval-augmented-generation)  
14. openai4rs \- Rust \- Docs.rs, Zugriff am Oktober 17, 2025, [https://docs.rs/openai4rs](https://docs.rs/openai4rs)  
15. Deploying a Complete RAG Ecosystem with a Single Command: My Ultimate Docker Stack | by Sebastian Torralba | Sep, 2025 \- FAUN.dev — Developer Community, Zugriff am Oktober 17, 2025, [https://faun.pub/deploying-a-complete-rag-ecosystem-with-a-single-command-my-ultimate-docker-stack-658e4421b74d](https://faun.pub/deploying-a-complete-rag-ecosystem-with-a-single-command-my-ultimate-docker-stack-658e4421b74d)  
16. Resource constraints \- Docker Docs, Zugriff am Oktober 17, 2025, [https://docs.docker.com/engine/containers/resource\_constraints/](https://docs.docker.com/engine/containers/resource_constraints/)  
17. Known limitations \- Meilisearch Documentation, Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/docs/learn/resources/known\_limitations](https://www.meilisearch.com/docs/learn/resources/known_limitations)  
18. Llama.cpp vs. Ollama: The Ultimate Comparison \- Arsturn, Zugriff am Oktober 17, 2025, [https://www.arsturn.com/blog/llama-cpp-vs-ollama-which-one-to-choose](https://www.arsturn.com/blog/llama-cpp-vs-ollama-which-one-to-choose)  
19. docker/genai-stack: Langchain \+ Docker \+ Neo4j \+ Ollama \- GitHub, Zugriff am Oktober 17, 2025, [https://github.com/docker/genai-stack](https://github.com/docker/genai-stack)  
20. Why you shouldn't use vector databases for RAG \- Meilisearch, Zugriff am Oktober 17, 2025, [https://www.meilisearch.com/blog/vector-dbs-rag](https://www.meilisearch.com/blog/vector-dbs-rag)  
21. meilisearch\_sdk \- Rust \- Docs.rs, Zugriff am Oktober 17, 2025, [https://docs.rs/meilisearch-sdk](https://docs.rs/meilisearch-sdk)  
22. meilisearch-sdk \- crates.io: Rust Package Registry, Zugriff am Oktober 17, 2025, [https://crates.io/crates/meilisearch-sdk](https://crates.io/crates/meilisearch-sdk)  
23. Finding the Best Open-Source Embedding Model for RAG \- TigerData, Zugriff am Oktober 17, 2025, [https://www.tigerdata.com/blog/finding-the-best-open-source-embedding-model-for-rag](https://www.tigerdata.com/blog/finding-the-best-open-source-embedding-model-for-rag)  
24. I Switched From Ollama And LM Studio To llama.cpp And Absolutely Loving It \- It's FOSS, Zugriff am Oktober 17, 2025, [https://itsfoss.com/llama-cpp/](https://itsfoss.com/llama-cpp/)  
25. markdown-parser \- crates.io: Rust Package Registry, Zugriff am Oktober 17, 2025, [https://crates.io/crates/markdown-parser](https://crates.io/crates/markdown-parser)  
26. extractous \- crates.io: Rust Package Registry, Zugriff am Oktober 17, 2025, [https://crates.io/crates/extractous](https://crates.io/crates/extractous)  
27. oxidize\_pdf \- Rust \- Docs.rs, Zugriff am Oktober 17, 2025, [https://docs.rs/oxidize-pdf/latest/oxidize\_pdf/](https://docs.rs/oxidize-pdf/latest/oxidize_pdf/)  
28. Rust RAG Toolchain \- Crates.io, Zugriff am Oktober 17, 2025, [https://crates.io/crates/rag-toolchain](https://crates.io/crates/rag-toolchain)  
29. llm\_utils \- crates.io: Rust Package Registry, Zugriff am Oktober 17, 2025, [https://crates.io/crates/llm\_utils](https://crates.io/crates/llm_utils)  
30. Step-By-Step Process of Building an Efficient RAG Workflow \- ChatBees, Zugriff am Oktober 17, 2025, [https://www.chatbees.ai/blog/rag-workflow](https://www.chatbees.ai/blog/rag-workflow)