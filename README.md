# RAG Assistant - Vue 3 + TypeScript + Vite

Ein RAG-Assistent mit Vue.js Frontend für die Verwaltung und Abfrage von Wissensdatenbanken.

## Features

- **Chat-Assistent**: Interaktive Abfrage der Wissensdatenbank
- **Wissensverwaltung**: Upload von Dokumenten und manuelle Eingabe
- **Markdown-Editor**: Split-Screen Editor mit Live-Vorschau
- **Task-Management**: Überwachung der Indexierungsprozesse

## Technologie-Stack

- Vue 3 mit TypeScript
- Vite als Build-Tool
- pnpm als Package Manager
- vue-markdown-render für Markdown-Darstellung
- Vue Router für Navigation

## Entwicklung

### Voraussetzungen

- Node.js (Version 18 oder höher)
- pnpm (Package Manager)

### Installation

```bash
# Dependencies installieren
pnpm install

# Entwicklungsserver starten
pnpm dev

# Build für Produktion
pnpm build
```

### Package Manager

Dieses Projekt verwendet **pnpm** als Package Manager. Stellen Sie sicher, dass pnpm installiert ist:

```bash
npm install -g pnpm
```

Die `package-lock.json` wird nicht verwendet. Stattdessen wird `pnpm-lock.yaml` für die Dependency-Verwaltung verwendet.
