# HoneyTrap Code Documentation

All code is documented following **Google Style Guide** with comprehensive examples in English.

## 📚 Documentation Standards

### File-level Documentation

Each file includes:

- `@fileoverview` - Description of the file's purpose
- Usage examples with `@example` tags
- Module-level documentation

### Function/Component Documentation

Each function includes:

- Description of functionality
- `@param` tags for parameters with types
- `@returns` tag for return values
- `@throws` tags for errors
- `@example` tags with practical usage examples

### Interface Documentation

Each interface includes:

- Description of the interface purpose
- `@interface` tag
- `@property` tags for each property with type and description
- `@example` tag showing usage

## 📂 Documented Files

### Web UI (`web-ui/src/`)

#### API Client (`api/client.ts`)

```typescript
/**
 * @fileoverview HoneyTrap API client for communication with the backend.
 * Provides type-safe methods for fetching dashboard data, connection stats,
 * honeypot sessions, ML metrics, and system information.
 */
```

**Interfaces:**

- `ConnectionStats` - Connection statistics (total, active, anomaly, normal, blocked)
- `HoneypotSession` - Session details (id, type, duration, credentials, commands)
- `MLMetrics` - ML model metrics (predictions, scores, inference time)
- `SystemMetrics` - System info (uptime, memory, CPU, tasks)
- `DashboardData` - Complete dashboard data

**Methods:**

- `getDashboardData()` - Fetch complete dashboard data
- `getConnectionStats()` - Get connection statistics
- `getHoneypotSessions(limit)` - Get recent sessions
- `getSessionDetails(id)` - Get session by ID
- `getMLMetrics()` - Get ML metrics
- `getSystemMetrics()` - Get system metrics
- `getPrometheusMetrics()` - Get raw Prometheus metrics
- `healthCheck()` - Health check endpoint

**Examples:**

```typescript
// Fetch dashboard data
const data = await honeytrapApi.getDashboardData();
console.log(`Total connections: ${data.connections.total}`);

// Get recent sessions
const sessions = await honeytrapApi.getHoneypotSessions(10);
sessions.forEach((s) => console.log(`${s.serviceType}: ${s.sourceIp}`));

// Calculate anomaly rate
const stats = await honeytrapApi.getConnectionStats();
const rate = ((stats.anomaly / stats.total) * 100).toFixed(1);
console.log(`Anomaly rate: ${rate}%`);
```

#### Main Entry (`main.tsx`)

```typescript
/**
 * @fileoverview Application entry point for HoneyTrap Web UI.
 * Sets up React rendering with routing and global styles.
 */
```

#### App Router (`App.tsx`)

```typescript
/**
 * @fileoverview Main application component with routing configuration.
 * Defines all routes and navigation structure for the HoneyTrap dashboard.
 */
```

**Routes:**

- `/` → Redirects to `/dashboard`
- `/dashboard` → Main dashboard with stats and charts
- `/connections` → Connection monitoring
- `/sessions` → Honeypot session viewer
- `/ml` → ML metrics and performance

#### Layout (`components/Layout.tsx`)

```typescript
/**
 * @fileoverview Main layout component with sidebar navigation.
 * Provides consistent layout structure across all pages with responsive sidebar.
 */
```

**Navigation Items:**

- Dashboard (Activity icon)
- Connections (Shield icon)
- Sessions (Database icon)
- ML Metrics (Brain icon)

#### Dashboard (`pages/Dashboard.tsx`)

```typescript
/**
 * @fileoverview Main dashboard page with real-time metrics and visualizations.
 * Displays connection statistics, honeypot sessions, and system metrics.
 */
```

**Features:**

- Auto-refresh every 5 seconds
- 4 stat cards (Total, Active, Anomaly, CPU)
- Pie chart - Connection distribution
- Bar chart - Session types
- Recent sessions table

**Components:**

- `Dashboard()` - Main dashboard component
- `StatCard()` - Statistic card with icon

#### Other Pages

- `Connections.tsx` - Connection monitoring (placeholder)
- `Sessions.tsx` - Session viewer (placeholder)
- `MLMetrics.tsx` - ML metrics (placeholder)

### Mobile App (`mobile-app/`)

#### API Client (`src/api/client.ts`)

```typescript
/**
 * @fileoverview HoneyTrap API client for React Native mobile app.
 * Provides type-safe methods for fetching dashboard data, connection stats,
 * honeypot sessions, ML metrics, and system information.
 */
```

Same interfaces and methods as Web UI, adapted for React Native.

**Configuration:**

```typescript
const API_BASE_URL = "http://your-server:8443/api";
const METRICS_BASE_URL = "http://your-server:9090";
```

#### Root Layout (`app/_layout.tsx`)

```typescript
/**
 * @fileoverview Root layout for HoneyTrap mobile app.
 * Wraps the app with SafeAreaProvider for proper spacing on all devices.
 */
```

#### Tab Layout (`app/(tabs)/_layout.tsx`)

```typescript
/**
 * @fileoverview Tab navigation layout for HoneyTrap mobile app.
 * Defines bottom tab navigation with 4 screens: Dashboard, Connections, Sessions, ML.
 */
```

**Tabs:**

1. **Dashboard** (index) - `speedometer` icon
2. **Connections** - `shield` icon
3. **Sessions** - `server` icon
4. **ML Metrics** - `analytics` icon

#### Dashboard (`app/(tabs)/index.tsx`)

```typescript
/**
 * @fileoverview Main dashboard screen for HoneyTrap mobile app.
 * Displays real-time metrics, charts, and recent sessions with pull-to-refresh.
 */
```

**Features:**

- Auto-refresh every 10 seconds
- Pull-to-refresh gesture
- 4 stat cards with colored borders
- Pie chart with react-native-chart-kit
- System metrics grid
- Recent sessions list

**Components:**

- `Dashboard()` - Main dashboard screen
- `StatCard()` - Stat card with colored border
- `MetricItem()` - Metric display item
- `SessionItem()` - Session list item

**Props Interfaces:**

- `StatCardProps` - title, value, color
- `MetricItemProps` - label, value
- `SessionItemProps` - session object

#### Other Screens

- `connections.tsx` - Connection monitoring (placeholder)
- `sessions.tsx` - Session viewer (placeholder)
- `ml.tsx` - ML metrics (placeholder)

## 🎯 Documentation Examples

### Interface with Example

```typescript
/**
 * Connection statistics aggregated across all connections.
 *
 * @interface ConnectionStats
 * @property {number} total - Total number of connections received
 * @property {number} active - Currently active connections
 * @property {number} anomaly - Connections classified as anomalous
 * @property {number} normal - Connections classified as normal
 * @property {number} blocked - Connections that were blocked
 *
 * @example
 * const stats: ConnectionStats = {
 *   total: 150,
 *   active: 12,
 *   anomaly: 8,
 *   normal: 140,
 *   blocked: 2
 * };
 */
export interface ConnectionStats {
  total: number;
  active: number;
  anomaly: number;
  normal: number;
  blocked: number;
}
```

### Method with Example

```typescript
/**
 * Fetches complete dashboard data including connections, sessions, ML metrics, and system info.
 *
 * @returns {Promise<DashboardData>} Complete dashboard data
 * @throws {Error} If the API request fails
 *
 * @example
 * const data = await honeytrapApi.getDashboardData();
 * console.log(`Total connections: ${data.connections.total}`);
 * console.log(`Anomaly rate: ${(data.connections.anomaly / data.connections.total * 100).toFixed(1)}%`);
 */
async getDashboardData(): Promise<DashboardData> {
  const response = await api.get('/dashboard');
  return response.data;
}
```

### Component with Example

```typescript
/**
 * Statistic card component with colored left border.
 *
 * @param {StatCardProps} props - Component props
 * @returns {JSX.Element} Stat card with title and value
 *
 * @example
 * <StatCard
 *   title="Active Connections"
 *   value={42}
 *   color="#10b981"
 * />
 */
function StatCard({ title, value, color }: StatCardProps) {
  return (
    <View style={[styles.statCard, { borderLeftColor: color }]}>
      <Text style={styles.statTitle}>{title}</Text>
      <Text style={styles.statValue}>{value}</Text>
    </View>
  );
}
```

## 📖 Documentation Tools

### Generating Documentation

Use JSDoc or TypeDoc to generate HTML documentation:

```bash
# Install TypeDoc
npm install -g typedoc

# Generate docs for Web UI
cd web-ui
typedoc --out docs src/

# Generate docs for Mobile App
cd mobile-app
typedoc --out docs src/ app/
```

### VSCode IntelliSense

All documentation is compatible with VSCode IntelliSense:

- Hover over functions to see documentation
- Autocomplete shows parameter types
- Examples appear in documentation popups

### IDE Support

Documentation works in:

- ✅ VSCode
- ✅ WebStorm/IntelliJ
- ✅ Sublime Text (with LSP)
- ✅ Vim/Neovim (with CoC or LSP)

## 🔍 Documentation Best Practices

### Do's

- ✅ Use `@example` tags with realistic use cases
- ✅ Include `@param` types and descriptions
- ✅ Document edge cases and error conditions
- ✅ Keep examples concise and practical
- ✅ Update docs when code changes

### Don'ts

- ❌ Don't document obvious code
- ❌ Don't use generic examples
- ❌ Don't skip error documentation
- ❌ Don't let docs go stale

## 📝 Documentation Checklist

- [x] All API methods documented
- [x] All interfaces documented with examples
- [x] All components documented
- [x] All props interfaces documented
- [x] File-level overviews added
- [x] Usage examples provided
- [x] Error handling documented
- [x] Google Style Guide followed
- [x] English language used throughout

## 🎓 Resources

- [Google TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
- [JSDoc Documentation](https://jsdoc.app/)
- [TypeDoc Documentation](https://typedoc.org/)
- [TSDoc Standard](https://tsdoc.org/)

## 🔗 Related Documentation

- `README.md` - Project overview and setup
- `UI_GUIDE.md` - Comprehensive UI guide
- `web-ui/README.md` - Web UI documentation
- `mobile-app/README.md` - Mobile app documentation
- `grafana/README.md` - Grafana dashboard documentation

---

**Note:** All code documentation follows Google Style Guide and includes practical examples in English. This ensures consistent, professional documentation across the entire codebase.
