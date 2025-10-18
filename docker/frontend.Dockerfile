# Frontend Dockerfile - VueJS mit Vite (Production)
FROM node:22-alpine AS builder

# Pnpm installieren
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Dependencies installieren
COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./
COPY ./apps/frontend/package.json ./apps/frontend/
RUN pnpm install --frozen-lockfile

WORKDIR /app/apps/frontend

# Source Code kopieren
COPY ./apps/frontend .

# Production Build
RUN pnpm build

# Production Stage mit nginx
FROM nginx:alpine AS production

# Custom nginx config kopieren
COPY --from=builder /app/apps/frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Build Output kopieren
COPY --from=builder /app/apps/frontend/dist /usr/share/nginx/html

# Health Check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:80/ || exit 1

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
