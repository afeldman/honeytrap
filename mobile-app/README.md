# HoneyTrap Mobile App

React Native mobile app for monitoring HoneyTrap on iOS and Android.

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- npm or yarn
- Expo CLI
- iOS Simulator (Mac) or Android Studio

### Installation

```bash
cd mobile-app
npm install
```

### Development

```bash
# Start Expo dev server
npm start

# Run on iOS simulator
npm run ios

# Run on Android emulator
npm run android

# Run on web (for testing)
npm run web
```

### Expo Go App

1. Install Expo Go on your phone:

   - iOS: https://apps.apple.com/app/expo-go/id982107779
   - Android: https://play.google.com/store/apps/details?id=host.exp.exponent

2. Scan QR code from `npm start`

## 📁 Project Structure

```
mobile-app/
├── app/                 # Expo Router pages
│   ├── (tabs)/         # Tab navigation
│   │   ├── index.tsx   # Dashboard
│   │   ├── connections.tsx
│   │   ├── sessions.tsx
│   │   └── ml.tsx
│   └── _layout.tsx     # Root layout
├── src/
│   └── api/            # API client
└── app.json            # Expo config
```

## 🎨 Features

- **Real-time Dashboard**: Live metrics and statistics
- **Connection Monitoring**: View all connections
- **Session Viewer**: Honeypot session details
- **ML Metrics**: AI/ML performance
- **Dark Theme**: Eye-friendly interface
- **Pull to Refresh**: Update data
- **Auto Refresh**: Every 10 seconds

## 🔧 Configuration

Edit `src/api/client.ts` to set your HoneyTrap server URL:

```typescript
const API_BASE_URL = "http://your-server:8443/api";
const METRICS_BASE_URL = "http://your-server:9090";
```

### For Local Development

If running on physical device, use your computer's local IP:

```typescript
const API_BASE_URL = "http://192.168.1.100:8443/api";
```

## 📱 Platform Support

- ✅ iOS 13.0+
- ✅ Android 5.0+ (API 21+)
- ✅ Web (for testing)

## 🎯 Tech Stack

- **Expo 50** - Framework
- **React Native** - Mobile UI
- **TypeScript** - Type safety
- **Expo Router** - File-based routing
- **React Native Chart Kit** - Charts
- **Axios** - HTTP client

## 📊 Screens

### Dashboard

- Connection statistics (total, active, anomaly)
- System metrics (CPU, memory, uptime)
- Connection distribution pie chart
- Recent honeypot sessions

### Connections

Real-time connection monitoring (coming soon)

### Sessions

Detailed honeypot session viewer (coming soon)

### ML Metrics

Machine learning performance metrics (coming soon)

## 🚢 Build & Deploy

### iOS

```bash
# Install EAS CLI
npm install -g eas-cli

# Login to Expo
eas login

# Configure build
eas build:configure

# Build for iOS
eas build --platform ios

# Submit to App Store
eas submit --platform ios
```

### Android

```bash
# Build APK for testing
eas build --platform android --profile preview

# Build AAB for Play Store
eas build --platform android

# Submit to Play Store
eas submit --platform android
```

## 🔒 Security

- API authentication required in production
- HTTPS recommended
- Rate limiting on API
- Secure storage for credentials

## 🐛 Troubleshooting

### Can't connect to API

1. Check server URL in `src/api/client.ts`
2. Ensure server is running
3. Use local IP for physical devices
4. Check firewall/network settings

### Expo Go Issues

1. Ensure phone and computer on same network
2. Try restarting Expo dev server
3. Clear Expo cache: `npx expo start -c`

## 📝 License

MIT License
