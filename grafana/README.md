# HoneyTrap Grafana Dashboard

Comprehensive monitoring dashboard for HoneyTrap.

## 📊 Dashboard Features

### Overview Panels

- **Total Connections** - Cumulative connection count
- **Active Connections** - Real-time active connections
- **Anomaly Connections** - Detected anomaly count
- **CPU Usage** - System CPU utilization

### Connection Analytics

- **Connection Rate** - Connections per second over time
- **Connections by Result** - Distribution (normal/anomaly/blocked)

### Honeypot Activity

- **Sessions by Type** - SSH, HTTP, MySQL session counts
- **Captured Credentials** - Table of credential captures by service

### ML & AI Metrics

- **Anomaly Score Distribution** - p50 and p95 anomaly scores
- **RL Agent Actions** - Distribution of agent decisions
- **ML Inference Duration** - Model performance metrics

### System Monitoring

- **Memory Usage** - System memory consumption

## 🚀 Quick Start

### 1. Start Prometheus

```bash
# prometheus.yml
scrape_configs:
  - job_name: 'honeytrap'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 15s
```

```bash
prometheus --config.file=prometheus.yml
```

### 2. Start Grafana

```bash
# Using Docker
docker run -d \
  -p 3000:3000 \
  --name=grafana \
  grafana/grafana-oss

# Or install locally
brew install grafana
brew services start grafana
```

### 3. Import Dashboard

1. Open Grafana: http://localhost:3000 (admin/admin)
2. Configuration → Data Sources → Add Prometheus
   - URL: http://localhost:9090
3. Dashboards → Import
4. Upload `honeytrap-dashboard.json`

## 📈 Dashboard Panels

### Panel 1-4: Overview Stats

Quick at-a-glance metrics with color-coded thresholds.

### Panel 5: Connection Rate

Time series showing connection rate with mean, last, and max values.

### Panel 6: Connections by Result

Donut chart showing distribution of connection classifications.

### Panel 7: Honeypot Sessions

Stacked bar chart of sessions by service type (SSH, HTTP, MySQL).

### Panel 8: Captured Credentials

Table showing credential capture counts by service type.

### Panel 9: Anomaly Score Distribution

Line chart with p50 and p95 percentiles for anomaly detection scores.

### Panel 10: RL Agent Actions

Stacked bar chart showing distribution of RL agent decisions.

### Panel 11: ML Inference Duration

Performance monitoring for ML model inference times.

### Panel 12: Memory Usage

System memory consumption over time.

## 🎨 Customization

### Adding Alerts

1. Edit panel → Alert tab
2. Create alert rule
3. Configure notification channel

Example alert for high anomaly rate:

```yaml
Alert: High Anomaly Rate
Condition: WHEN sum() OF query(A, 5m, now) IS ABOVE 10
Frequency: 1m
For: 5m
```

### Adding Panels

Use existing panels as templates:

```json
{
  "datasource": { "type": "prometheus", "uid": "${DS_PROMETHEUS}" },
  "targets": [
    {
      "expr": "your_metric_here",
      "refId": "A"
    }
  ]
}
```

## 📱 Mobile View

Dashboard is responsive and works on mobile devices. Consider creating a simplified mobile dashboard for better UX.

## 🔧 Troubleshooting

### No Data Showing

1. Verify HoneyTrap metrics exporter is running:

   ```bash
   curl http://localhost:9090/metrics
   ```

2. Check Prometheus targets:
   http://localhost:9090/targets

3. Verify data source in Grafana:
   Configuration → Data Sources → Prometheus → Test

### Slow Dashboard

- Reduce time range (default: 1h)
- Increase scrape interval
- Use recording rules for complex queries

## 📚 Resources

- [Grafana Documentation](https://grafana.com/docs/)
- [Prometheus Query Language](https://prometheus.io/docs/prometheus/latest/querying/basics/)
- [HoneyTrap Metrics Documentation](../crates/honeytrap-metrics/README.md)
