# HoneyTrap Web UI

Modern React-based dashboard for monitoring HoneyTrap.

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
cd web-ui
npm install
```

### Development

```bash
npm run dev
```

Open http://localhost:3001

### Production Build

```bash
npm run build
npm run preview
```

### Docker

```bash
# Build Docker image
docker build -t honeytrap-ui:latest .

# Run container
docker run -d -p 3001:80 --name honeytrap-ui honeytrap-ui:latest

# Using Docker Compose
docker-compose up -d

# View logs
docker logs -f honeytrap-ui
```

See `DOCKER.md` for comprehensive Docker documentation.

## 📁 Project Structure

```
web-ui/
├── src/
│   ├── api/              # API client
│   ├── components/       # React components
│   ├── pages/           # Page components
│   ├── App.tsx          # App router
│   └── main.tsx         # Entry point
├── public/              # Static assets
└── package.json         # Dependencies
```

## 🎨 Features

- **Real-time Dashboard**: Live metrics and statistics
- **Connection Monitoring**: View all connections and their classification
- **Session Viewer**: Detailed honeypot session information
- **ML Metrics**: Machine learning performance monitoring
- **Dark Theme**: Modern, eye-friendly interface
- **Responsive Design**: Works on desktop and mobile

## 🔧 Configuration

### API Proxy

Configure in `vite.config.ts`:

```typescript
server: {
  proxy: {
    '/api': 'http://localhost:8443',
    '/metrics': 'http://localhost:9090',
  },
}
```

### Environment Variables

Create `.env.local`:

```env
VITE_API_URL=http://localhost:8443
VITE_METRICS_URL=http://localhost:9090
```

## 🎯 Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool
- **TailwindCSS** - Styling
- **Recharts** - Data visualization
- **React Router** - Navigation
- **Axios** - HTTP client
- **Lucide React** - Icons

## 📊 API Integration

The UI connects to HoneyTrap's REST API:

- `GET /api/dashboard` - Dashboard data
- `GET /api/stats/connections` - Connection statistics
- `GET /api/sessions` - Honeypot sessions
- `GET /api/stats/ml` - ML metrics
- `GET /metrics` - Prometheus metrics

## 🚢 Deployment

### Docker (Recommended)

Complete Docker setup with multi-stage build and nginx:

```bash
# Quick start
docker build -t honeytrap-ui:latest .
docker run -d -p 80:80 honeytrap-ui:latest

# Production with Docker Compose
docker-compose up -d

# Scale with Docker Swarm
docker service create --name honeytrap-ui --publish 80:80 --replicas 3 honeytrap-ui:latest
```

**Features:**

- Multi-stage build (~25MB final image)
- Nginx with reverse proxy to backend
- Health checks included
- Gzip compression
- Security headers
- Production-ready configuration

See **DOCKER.md** for complete documentation.

### Manual Nginx Setup

```nginx
server {
    listen 80;
    server_name honeytrap.example.com;

    location / {
        root /var/www/honeytrap-ui;
        try_files $uri /index.html;
    }

    location /api {
        proxy_pass http://localhost:8443;
    }

    location /metrics {
        proxy_pass http://localhost:9090;
    }
}
```

## 🔒 Security

- API authentication required in production
- CORS configuration needed
- Use HTTPS in production
- Rate limiting recommended

## 📝 License

MIT License
