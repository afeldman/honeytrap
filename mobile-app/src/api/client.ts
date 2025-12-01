/**
 * @fileoverview HoneyTrap API client for React Native mobile app.
 * Provides type-safe methods for fetching dashboard data, connection stats,
 * honeypot sessions, ML metrics, and system information.
 *
 * @example
 * import { honeytrapApi } from '../src/api/client';
 *
 * // Fetch dashboard data
 * const data = await honeytrapApi.getDashboardData();
 * console.log(`Total connections: ${data.connections.total}`);
 *
 * @example
 * // Get connection stats only
 * const stats = await honeytrapApi.getConnectionStats();
 * console.log(`Active: ${stats.active}, Anomaly: ${stats.anomaly}`);
 *
 * @example
 * // Health check
 * const health = await honeytrapApi.healthCheck();
 * console.log(`Status: ${health.status}`);
 */

import axios from "axios";

// Change this to your HoneyTrap server URL
const API_BASE_URL = "http://your-server:8443/api";
const METRICS_BASE_URL = "http://your-server:9090";

const api = axios.create({
    baseURL: API_BASE_URL,
    timeout: 10000,
});

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

export interface HoneypotSession {
    id: string;
    serviceType: "ssh" | "http" | "mysql";
    startTime: string;
    duration: number;
    credentialsCaptured: number;
    commandsExecuted: number;
    maliciousCommands: number;
    sourceIp: string;
    status: "active" | "closed";
}

export interface MLMetrics {
    totalPredictions: number;
    randomforestPredictions: number;
    rlAgentPredictions: number;
    avgAnomalyScore: number;
    avgInferenceTime: number;
}

export interface SystemMetrics {
    uptime: number;
    memoryUsage: number;
    cpuUsage: number;
    activeTasks: number;
}

export interface DashboardData {
    connections: ConnectionStats;
    honeypotSessions: HoneypotSession[];
    mlMetrics: MLMetrics;
    systemMetrics: SystemMetrics;
}

export const honeytrapApi = {
    async getDashboardData(): Promise<DashboardData> {
        const response = await api.get("/dashboard");
        return response.data;
    },

    async getConnectionStats(): Promise<ConnectionStats> {
        const response = await api.get("/stats/connections");
        return response.data;
    },

    async getHoneypotSessions(limit = 50): Promise<HoneypotSession[]> {
        const response = await api.get("/sessions", { params: { limit } });
        return response.data;
    },

    async getMLMetrics(): Promise<MLMetrics> {
        const response = await api.get("/stats/ml");
        return response.data;
    },

    async getSystemMetrics(): Promise<SystemMetrics> {
        const response = await api.get("/stats/system");
        return response.data;
    },

    async healthCheck(): Promise<{ status: string }> {
        const response = await axios.get(`${METRICS_BASE_URL}/health`);
        return response.data;
    },
};

export default api;
