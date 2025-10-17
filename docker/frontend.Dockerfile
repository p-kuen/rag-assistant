# Frontend Dockerfile - VueJS mit Vite (Production)
FROM node:20-alpine AS builder

# Pnpm installieren
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Dependencies installieren
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Source Code kopieren
COPY . .

# Production Build
RUN pnpm build

# Production Stage mit nginx
FROM nginx:alpine AS production

# Custom nginx config kopieren
COPY --from=builder /app/nginx.conf /etc/nginx/conf.d/default.conf

# Build Output kopieren
COPY --from=builder /app/dist /usr/share/nginx/html

# Health Check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --quiet --tries=1 --spider http://localhost:80/ || exit 1

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
