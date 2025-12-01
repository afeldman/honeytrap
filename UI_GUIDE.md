# HoneyTrap UI & Dashboard Suite

Complete monitoring and visualization suite for HoneyTrap.

## 📊 Overview

Three complementary interfaces for monitoring HoneyTrap:

1. **Grafana Dashboard** - Professional metrics visualization
2. **Web UI** - Interactive React dashboard
3. **Mobile App** - On-the-go monitoring (iOS/Android)

## 🎯 Quick Start Guide

### 1. Grafana Dashboard

**Setup:**

```bash
# Start Prometheus
prometheus --config.file=prometheus.yml

# Start Grafana
docker run -d -p 3000:3000 grafana/grafana-oss

# Import dashboard
# Go to http://localhost:3000
# Upload: grafana/honeytrap-dashboard.json
```

**Features:**

- 12 comprehensive panels
- Real-time metrics from Prometheus
- Connection analysis & distribution
- Honeypot activity monitoring
- ML model performance tracking
- System resource monitoring

### 2. Web UI (React)

**Setup:**

```bash
cd web-ui
npm install
npm run dev
# Open http://localhost:3001
```

**Features:**

- Real-time dashboard with live updates (5s)
- Connection monitoring & classification
- Honeypot session viewer
- ML metrics visualization
- Dark theme, responsive design
- Recharts for data visualization

**Tech Stack:**

- React 18 + TypeScript
- Vite (build tool)
- TailwindCSS (styling)
- React Router (navigation)
- Axios (API client)

### 3. Mobile App (React Native)

**Setup:**

```bash
cd mobile-app
npm install
npm start
# Scan QR code with Expo Go app
```

**Features:**

- Real-time monitoring
- Pull-to-refresh (10s auto-refresh)
- Connection statistics
- System metrics
- Session monitoring
- Dark theme optimized for mobile

**Platforms:**

- iOS 13.0+
- Android 5.0+ (API 21+)
- Built with Expo 50

## 📁 Project Structure

```
honeytrap/
├── grafana/
│   ├── honeytrap-dashboard.json    # Dashboard template (12 panels)
│   └── README.md                   # Grafana setup guide
├── web-ui/
│   ├── src/
│   │   ├── api/                    # API client
│   │   ├── components/             # React components
│   │   ├── pages/                  # Dashboard, Connections, Sessions, ML
│   │   └── App.tsx                 # Router
│   ├── package.json
│   └── README.md
└── mobile-app/
    ├── app/
    │   └── (tabs)/                 # Tab navigation
    │       ├── index.tsx           # Dashboard
    │       ├── connections.tsx
    │       ├── sessions.tsx
    │       └── ml.tsx
    ├── src/api/                    # API client
    ├── package.json
    └── README.md
```

## 🔌 API Integration

All UIs connect to HoneyTrap's REST API:

```typescript
// Required API endpoints:
GET /api/dashboard              // Dashboard summary
GET /api/stats/connections      // Connection statistics
GET /api/sessions               // Honeypot sessions
GET /api/sessions/:id           // Session details
GET /api/stats/ml               // ML metrics
GET /api/stats/system           // System metrics
GET /metrics                    // Prometheus metrics (raw)
GET /metrics/health             // Health check
```

## 🚀 Deployment

### Grafana Dashboard

```bash
# Production deployment
docker-compose up -d grafana prometheus

# Or use existing Grafana instance
# Import grafana/honeytrap-dashboard.json via UI
```

### Web UI

```bash
# Build for production
cd web-ui
npm run build

# Deploy with nginx
cp -r dist/* /var/www/honeytrap-ui/

# Or use Docker
docker build -t honeytrap-ui .
docker run -p 80:80 honeytrap-ui
```

### Mobile App

```bash
# Install EAS CLI
npm install -g eas-cli

# Build iOS
cd mobile-app
eas build --platform ios

# Build Android
eas build --platform android

# Submit to stores
eas submit --platform ios
eas submit --platform android
```

## 🎨 Features Comparison

| Feature               | Grafana     | Web UI  | Mobile App |
| --------------------- | ----------- | ------- | ---------- |
| Real-time updates     | ✅ (scrape) | ✅ (5s) | ✅ (10s)   |
| Connection monitoring | ✅          | ✅      | ✅         |
| Honeypot sessions     | ✅          | ✅      | ✅         |
| ML metrics            | ✅          | ✅      | ✅         |
| System metrics        | ✅          | ✅      | ✅         |
| Historical data       | ✅          | ❌      | ❌         |
| Alerting              | ✅          | ❌      | 🚧         |
| Custom queries        | ✅          | ❌      | ❌         |
| Mobile optimized      | ❌          | ⚠️      | ✅         |
| Offline viewing       | ✅          | ❌      | ❌         |
| Push notifications    | ❌          | ❌      | 🚧         |

✅ = Available | ⚠️ = Partial | ❌ = Not available | 🚧 = Planned

## 🔧 Configuration

### Prometheus (for Grafana)

```yaml
# prometheus.yml
scrape_configs:
  - job_name: "honeytrap"
    static_configs:
      - targets: ["localhost:9090"]
    scrape_interval: 15s
```

### Web UI Proxy

```typescript
// web-ui/vite.config.ts
server: {
  proxy: {
    '/api': 'http://localhost:8443',
    '/metrics': 'http://localhost:9090',
  },
}
```

### Mobile App

```typescript
// mobile-app/src/api/client.ts
const API_BASE_URL = "http://your-server:8443/api";
const METRICS_BASE_URL = "http://your-server:9090";
```

## 📊 Metrics Overview

### Available Metrics

**Connection Metrics:**

- `honeytrap_connections_total` - Total connections
- `honeytrap_connections_active` - Active connections
- `honeytrap_connections_by_result` - By classification
- `honeytrap_connection_duration_seconds` - Duration histogram
- `honeytrap_bytes_total` - Bytes transferred

**Honeypot Metrics:**

- `honeytrap_honeypot_sessions_total` - Sessions by type
- `honeytrap_honeypot_sessions_active` - Active sessions
- `honeytrap_credentials_captured_total` - Captured credentials
- `honeytrap_commands_executed_total` - Executed commands
- `honeytrap_malicious_commands_total` - Malicious commands

**ML Metrics:**

- `honeytrap_ml_predictions_total` - Predictions by model
- `honeytrap_ml_inference_duration_seconds` - Inference time
- `honeytrap_anomaly_scores` - Score distribution
- `honeytrap_rl_actions_total` - RL agent actions
- `honeytrap_rl_q_values` - Q-values

**System Metrics:**

- `honeytrap_uptime_seconds` - Uptime
- `honeytrap_memory_bytes` - Memory usage
- `honeytrap_cpu_usage_percent` - CPU usage
- `honeytrap_active_tasks` - Active tasks

## 🔒 Security Considerations

### Production Deployment

1. **Authentication**: Add API authentication
2. **HTTPS**: Use TLS for all connections
3. **CORS**: Configure CORS properly
4. **Rate Limiting**: Implement rate limits
5. **API Keys**: Use API keys for mobile app
6. **Network Security**: Firewall rules for metrics endpoint

### Grafana Security

```yaml
# grafana.ini
[security]
admin_user = admin
admin_password = <strong-password>

[auth.anonymous]
enabled = false

[auth]
disable_login_form = false
```

## 🐛 Troubleshooting

### Grafana: No Data

1. Check Prometheus is running: `curl http://localhost:9090/metrics`
2. Verify Prometheus targets: http://localhost:9090/targets
3. Check data source in Grafana
4. Verify HoneyTrap metrics exporter is running

### Web UI: Can't Connect

1. Check API server is running on port 8443
2. Verify proxy configuration in `vite.config.ts`
3. Check browser console for errors
4. Test API directly: `curl http://localhost:8443/api/dashboard`

### Mobile App: Connection Failed

1. Check server URL in `src/api/client.ts`
2. Use local IP for physical devices (not localhost)
3. Ensure phone and computer on same network
4. Check firewall rules

## 📚 Documentation

- **Grafana**: See `grafana/README.md`
- **Web UI**: See `web-ui/README.md`
- **Mobile App**: See `mobile-app/README.md`
- **HoneyTrap Metrics**: See `crates/honeytrap-metrics/README.md`

## 🎯 Recommended Setup

For comprehensive monitoring:

1. **Development**: Web UI + Metrics demo
2. **Production**: Grafana + Prometheus + Alert Manager
3. **Mobile**: Expo Go app for on-the-go monitoring
4. **Team**: All three for complete visibility

## 📝 License

MIT License
