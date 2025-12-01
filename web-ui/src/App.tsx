/**
 * @fileoverview Main application component with routing configuration.
 * Defines all routes and navigation structure for the HoneyTrap dashboard.
 *
 * @example
 * // Routes defined:
 * // / → redirects to /dashboard
 * // /dashboard → Main dashboard with stats and charts
 * // /connections → Connection monitoring
 * // /sessions → Honeypot session viewer
 * // /ml → ML metrics and performance
 */

import { Routes, Route, Navigate } from "react-router-dom";
import Layout from "./components/Layout";
import Dashboard from "./pages/Dashboard";
import Connections from "./pages/Connections";
import Sessions from "./pages/Sessions";
import MLMetrics from "./pages/MLMetrics";

/**
 * Main application component with routing configuration.
 *
 * @returns {JSX.Element} App component with nested routes
 *
 * @example
 * // Usage in main.tsx:
 * <BrowserRouter>
 *   <App />
 * </BrowserRouter>
 */
function App() {
    return (
        <Routes>
            <Route path="/" element={<Layout />}>
                <Route index element={<Navigate to="/dashboard" replace />} />
                <Route path="dashboard" element={<Dashboard />} />
                <Route path="connections" element={<Connections />} />
                <Route path="sessions" element={<Sessions />} />
                <Route path="ml" element={<MLMetrics />} />
            </Route>
        </Routes>
    );
}

export default App;
