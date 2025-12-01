# HoneyTrap Web UI - Docker Setup

Docker configuration for the HoneyTrap Web UI React application.

## 🐳 Quick Start

### Build and Run

```bash
# Build the Docker image
docker build -t honeytrap-ui:latest .

# Run the container
docker run -d \
  -p 3001:80 \
  --name honeytrap-ui \
  honeytrap-ui:latest

# Open in browser
open http://localhost:3001
```

### Using Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f honeytrap-ui

# Stop services
docker-compose down
```

## 📦 Docker Image

### Multi-stage Build

The Dockerfile uses a multi-stage build for optimization:

1. **Builder stage** (node:18-alpine)

   - Installs dependencies
   - Builds the React application
   - Output: Production-ready static files

2. **Production stage** (nginx:alpine)
   - Copies built files from builder
   - Serves with nginx
   - Includes custom nginx configuration
   - Final image size: ~25MB

### Image Details

```
Repository: honeytrap-ui
Tag: latest
Size: ~25MB (compressed)
Base: nginx:alpine
```

## 🔧 Configuration

### Environment Variables

Pass environment variables at build time:

```bash
docker build \
  --build-arg VITE_API_URL=http://your-server:8443 \
  --build-arg VITE_METRICS_URL=http://your-server:9090 \
  -t honeytrap-ui:latest .
```

### Nginx Configuration

The `nginx.conf` includes:

- **Reverse Proxy**: `/api` → HoneyTrap backend
- **Metrics Proxy**: `/metrics` → Prometheus exporter
- **SPA Routing**: All routes redirect to `index.html`
- **Gzip Compression**: For all text-based content
- **Cache Control**: 1 year for static assets
- **Security Headers**: X-Frame-Options, X-XSS-Protection, etc.
- **Health Check**: `/health` endpoint

### Custom nginx.conf

Edit `nginx.conf` to customize:

```nginx
# Change backend server
location /api {
    proxy_pass http://your-backend:8443;
}

# Add CORS headers
add_header Access-Control-Allow-Origin "*" always;
```

## 🚀 Deployment

### Standalone Container

```bash
docker run -d \
  -p 80:80 \
  --name honeytrap-ui \
  --restart unless-stopped \
  honeytrap-ui:latest
```

### With Docker Compose

```yaml
version: "3.8"
services:
  honeytrap-ui:
    image: honeytrap-ui:latest
    ports:
      - "80:80"
    environment:
      - NODE_ENV=production
    restart: unless-stopped
```

### Docker Swarm

```bash
# Create service
docker service create \
  --name honeytrap-ui \
  --publish 80:80 \
  --replicas 3 \
  honeytrap-ui:latest

# Scale service
docker service scale honeytrap-ui=5
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: honeytrap-ui
spec:
  replicas: 3
  selector:
    matchLabels:
      app: honeytrap-ui
  template:
    metadata:
      labels:
        app: honeytrap-ui
    spec:
      containers:
        - name: honeytrap-ui
          image: honeytrap-ui:latest
          ports:
            - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: honeytrap-ui
spec:
  type: LoadBalancer
  ports:
    - port: 80
      targetPort: 80
  selector:
    app: honeytrap-ui
```

## 📊 Monitoring

### Health Check

Built-in health check:

```bash
# Check container health
docker inspect --format='{{.State.Health.Status}}' honeytrap-ui

# Manual health check
curl http://localhost:3001/health
```

### Logs

```bash
# View logs
docker logs honeytrap-ui

# Follow logs
docker logs -f honeytrap-ui

# Last 100 lines
docker logs --tail 100 honeytrap-ui
```

### Metrics

Access nginx metrics:

```bash
# Request count, response times via nginx logs
docker exec honeytrap-ui tail -f /var/log/nginx/access.log
```

## 🔒 Security

### Production Best Practices

1. **Use HTTPS**: Run behind reverse proxy with SSL
2. **Update Base Image**: Regularly update `nginx:alpine`
3. **Scan for Vulnerabilities**: Use `docker scan`
4. **Non-root User**: Nginx runs as `nginx` user
5. **Read-only Filesystem**: Add `--read-only` flag
6. **Drop Capabilities**: Use `--cap-drop ALL`

### Secure Run Command

```bash
docker run -d \
  -p 80:80 \
  --name honeytrap-ui \
  --restart unless-stopped \
  --read-only \
  --cap-drop ALL \
  --cap-add NET_BIND_SERVICE \
  --security-opt no-new-privileges:true \
  honeytrap-ui:latest
```

### Scan Image

```bash
# Docker scan
docker scan honeytrap-ui:latest

# Trivy scan
trivy image honeytrap-ui:latest
```

## 🛠️ Development

### Development Container

For development with hot-reload:

```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 3001
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
```

```bash
# Build dev image
docker build -f Dockerfile.dev -t honeytrap-ui:dev .

# Run with volume mount
docker run -d \
  -p 3001:3001 \
  -v $(pwd):/app \
  -v /app/node_modules \
  honeytrap-ui:dev
```

## 📝 Build Arguments

Available build arguments:

```bash
docker build \
  --build-arg NODE_VERSION=18 \
  --build-arg NGINX_VERSION=alpine \
  --build-arg BUILD_DATE=$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
  --build-arg VCS_REF=$(git rev-parse --short HEAD) \
  -t honeytrap-ui:latest .
```

## 🐛 Troubleshooting

### Container won't start

```bash
# Check logs
docker logs honeytrap-ui

# Inspect container
docker inspect honeytrap-ui

# Check nginx config
docker exec honeytrap-ui nginx -t
```

### API Connection Issues

```bash
# Test API proxy
docker exec honeytrap-ui wget -O- http://localhost/api/dashboard

# Check network
docker network inspect bridge
```

### Permission Issues

```bash
# Fix file permissions before build
chmod -R 755 .
```

## 📚 Additional Resources

- [Nginx Docker Documentation](https://hub.docker.com/_/nginx)
- [Node.js Docker Best Practices](https://github.com/nodejs/docker-node/blob/main/docs/BestPractices.md)
- [Docker Multi-stage Builds](https://docs.docker.com/build/building/multi-stage/)

## 🎯 Performance Tips

1. **Use CDN**: Serve static assets from CDN
2. **Enable HTTP/2**: Configure nginx for HTTP/2
3. **Optimize Images**: Compress images before build
4. **Lazy Loading**: Use React lazy loading
5. **Service Worker**: Implement PWA caching

## 📦 Registry

### Push to Docker Hub

```bash
# Tag image
docker tag honeytrap-ui:latest yourusername/honeytrap-ui:latest
docker tag honeytrap-ui:latest yourusername/honeytrap-ui:1.0.0

# Push to registry
docker push yourusername/honeytrap-ui:latest
docker push yourusername/honeytrap-ui:1.0.0
```

### Private Registry

```bash
# Tag for private registry
docker tag honeytrap-ui:latest registry.example.com/honeytrap-ui:latest

# Login and push
docker login registry.example.com
docker push registry.example.com/honeytrap-ui:latest
```

## 📄 License

MIT License
