import axios from "axios";

const api = axios.create({
    baseURL: "/api",
    timeout: 10000,
});

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
    // Dashboard data
    async getDashboardData(): Promise<DashboardData> {
        const response = await api.get("/dashboard");
        return response.data;
    },

    // Connection stats
    async getConnectionStats(): Promise<ConnectionStats> {
        const response = await api.get("/stats/connections");
        return response.data;
    },

    // Honeypot sessions
    async getHoneypotSessions(limit = 50): Promise<HoneypotSession[]> {
        const response = await api.get("/sessions", { params: { limit } });
        return response.data;
    },

    async getSessionDetails(sessionId: string): Promise<HoneypotSession> {
        const response = await api.get(`/sessions/${sessionId}`);
        return response.data;
    },

    // ML metrics
    async getMLMetrics(): Promise<MLMetrics> {
        const response = await api.get("/stats/ml");
        return response.data;
    },

    // System metrics
    async getSystemMetrics(): Promise<SystemMetrics> {
        const response = await api.get("/stats/system");
        return response.data;
    },

    // Prometheus metrics (raw)
    async getPrometheusMetrics(): Promise<string> {
        const response = await axios.get("/metrics");
        return response.data;
    },

    // Health check
    async healthCheck(): Promise<{ status: string }> {
        const response = await axios.get("/metrics/health");
        return response.data;
    },
};

export default api;
