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

### Docker

```bash
# Build image
docker build -t honeytrap-ui .

# Run container
docker run -p 3001:80 honeytrap-ui
```

### Nginx

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
