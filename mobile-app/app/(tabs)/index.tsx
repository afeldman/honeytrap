/**
 * @fileoverview Main dashboard screen for HoneyTrap mobile app.
 * Displays real-time metrics, charts, and recent sessions with pull-to-refresh.
 *
 * @example
 * // Dashboard features:
 * // - Auto-refresh every 10 seconds
 * // - Pull-to-refresh gesture
 * // - 4 stat cards (Total, Active, Anomaly, CPU)
 * // - Pie chart for connection distribution
 * // - System metrics grid
 * // - Recent sessions list
 */

import { useEffect, useState } from "react";
import {
    View,
    Text,
    StyleSheet,
    ScrollView,
    RefreshControl,
    ActivityIndicator,
} from "react-native";
import { honeytrapApi, DashboardData } from "../../src/api/client";
import { PieChart } from "react-native-chart-kit";
import { Dimensions } from "react-native";

const screenWidth = Dimensions.get("window").width;

/**
 * Main dashboard screen component.
 *
 * @returns {JSX.Element} Dashboard with stats, charts, and sessions
 *
 * @example
 * // Automatic updates:
 * // - useEffect fetches data on mount
 * // - setInterval refreshes every 10 seconds
 * // - Pull-to-refresh triggers manual update
 */
export default function Dashboard() {
    const [data, setData] = useState<DashboardData | null>(null);
    const [loading, setLoading] = useState(true);
    const [refreshing, setRefreshing] = useState(false);

    const fetchData = async () => {
        try {
            const dashboardData = await honeytrapApi.getDashboardData();
            setData(dashboardData);
        } catch (error) {
            console.error("Failed to fetch dashboard data:", error);
        } finally {
            setLoading(false);
            setRefreshing(false);
        }
    };

    useEffect(() => {
        fetchData();
        const interval = setInterval(fetchData, 10000);
        return () => clearInterval(interval);
    }, []);

    const onRefresh = () => {
        setRefreshing(true);
        fetchData();
    };

    if (loading) {
        return (
            <View style={styles.centered}>
                <ActivityIndicator size="large" color="#10b981" />
            </View>
        );
    }

    if (!data) {
        return (
            <View style={styles.centered}>
                <Text style={styles.errorText}>Failed to load data</Text>
            </View>
        );
    }

    const pieData = [
        {
            name: "Normal",
            value: data.connections.normal,
            color: "#10b981",
            legendFontColor: "#9ca3af",
        },
        {
            name: "Anomaly",
            value: data.connections.anomaly,
            color: "#f59e0b",
            legendFontColor: "#9ca3af",
        },
        {
            name: "Blocked",
            value: data.connections.blocked,
            color: "#ef4444",
            legendFontColor: "#9ca3af",
        },
    ];

    return (
        <ScrollView
            style={styles.container}
            refreshControl={
                <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
            }
        >
            <View style={styles.header}>
                <Text style={styles.title}>🍯 HoneyTrap</Text>
            </View>

            {/* Stats Cards */}
            <View style={styles.statsGrid}>
                <StatCard
                    title="Total Connections"
                    value={data.connections.total}
                    color="#3b82f6"
                />
                <StatCard
                    title="Active"
                    value={data.connections.active}
                    color="#10b981"
                />
                <StatCard
                    title="Anomaly"
                    value={data.connections.anomaly}
                    color="#ef4444"
                />
                <StatCard
                    title="CPU"
                    value={`${data.systemMetrics.cpuUsage.toFixed(1)}%`}
                    color="#f59e0b"
                />
            </View>

            {/* Connection Distribution Chart */}
            <View style={styles.card}>
                <Text style={styles.cardTitle}>Connection Distribution</Text>
                <PieChart
                    data={pieData}
                    width={screenWidth - 40}
                    height={220}
                    chartConfig={{
                        color: (opacity = 1) =>
                            `rgba(255, 255, 255, ${opacity})`,
                    }}
                    accessor="value"
                    backgroundColor="transparent"
                    paddingLeft="15"
                    absolute
                />
            </View>

            {/* System Metrics */}
            <View style={styles.card}>
                <Text style={styles.cardTitle}>System Metrics</Text>
                <View style={styles.metricsRow}>
                    <MetricItem
                        label="Uptime"
                        value={`${Math.floor(data.systemMetrics.uptime / 60)}m`}
                    />
                    <MetricItem
                        label="Memory"
                        value={`${(
                            data.systemMetrics.memoryUsage /
                            1024 /
                            1024
                        ).toFixed(0)}MB`}
                    />
                </View>
                <View style={styles.metricsRow}>
                    <MetricItem
                        label="Tasks"
                        value={data.systemMetrics.activeTasks.toString()}
                    />
                    <MetricItem
                        label="Sessions"
                        value={data.honeypotSessions.length.toString()}
                    />
                </View>
            </View>

            {/* Recent Sessions */}
            <View style={styles.card}>
                <Text style={styles.cardTitle}>Recent Sessions</Text>
                {data.honeypotSessions.slice(0, 5).map((session) => (
                    <SessionItem key={session.id} session={session} />
                ))}
            </View>
        </ScrollView>
    );
}

/**
 * Props for StatCard component.
 *
 * @interface StatCardProps
 * @property {string} title - Card title (e.g., "Total Connections")
 * @property {string | number} value - Displayed value
 * @property {string} color - Border color (hex or named)
 */
interface StatCardProps {
    title: string;
    value: string | number;
    color: string;
}

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

interface MetricItemProps {
    label: string;
    value: string;
}

function MetricItem({ label, value }: MetricItemProps) {
    return (
        <View style={styles.metricItem}>
            <Text style={styles.metricLabel}>{label}</Text>
            <Text style={styles.metricValue}>{value}</Text>
        </View>
    );
}

interface SessionItemProps {
    session: any;
}

function SessionItem({ session }: SessionItemProps) {
    return (
        <View style={styles.sessionItem}>
            <View style={styles.sessionHeader}>
                <Text style={styles.sessionType}>
                    {session.serviceType.toUpperCase()}
                </Text>
                <Text style={styles.sessionStatus}>{session.status}</Text>
            </View>
            <Text style={styles.sessionIp}>{session.sourceIp}</Text>
            <Text style={styles.sessionDetails}>
                {session.duration}s • {session.credentialsCaptured} creds •{" "}
                {session.commandsExecuted} cmds
            </Text>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#111827",
    },
    centered: {
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
        backgroundColor: "#111827",
    },
    header: {
        padding: 20,
        paddingTop: 10,
    },
    title: {
        fontSize: 28,
        fontWeight: "bold",
        color: "#fff",
    },
    errorText: {
        color: "#ef4444",
        fontSize: 16,
    },
    statsGrid: {
        flexDirection: "row",
        flexWrap: "wrap",
        paddingHorizontal: 10,
        gap: 10,
    },
    statCard: {
        flex: 1,
        minWidth: "45%",
        backgroundColor: "#1f2937",
        borderRadius: 12,
        padding: 15,
        borderLeftWidth: 4,
    },
    statTitle: {
        color: "#9ca3af",
        fontSize: 12,
        marginBottom: 5,
    },
    statValue: {
        color: "#fff",
        fontSize: 24,
        fontWeight: "bold",
    },
    card: {
        margin: 10,
        marginTop: 20,
        backgroundColor: "#1f2937",
        borderRadius: 12,
        padding: 15,
    },
    cardTitle: {
        color: "#fff",
        fontSize: 18,
        fontWeight: "600",
        marginBottom: 15,
    },
    metricsRow: {
        flexDirection: "row",
        justifyContent: "space-around",
        marginVertical: 10,
    },
    metricItem: {
        alignItems: "center",
    },
    metricLabel: {
        color: "#9ca3af",
        fontSize: 12,
        marginBottom: 5,
    },
    metricValue: {
        color: "#fff",
        fontSize: 20,
        fontWeight: "600",
    },
    sessionItem: {
        backgroundColor: "#374151",
        borderRadius: 8,
        padding: 12,
        marginBottom: 10,
    },
    sessionHeader: {
        flexDirection: "row",
        justifyContent: "space-between",
        marginBottom: 5,
    },
    sessionType: {
        color: "#10b981",
        fontSize: 14,
        fontWeight: "600",
    },
    sessionStatus: {
        color: "#9ca3af",
        fontSize: 12,
    },
    sessionIp: {
        color: "#fff",
        fontSize: 14,
        marginBottom: 3,
    },
    sessionDetails: {
        color: "#9ca3af",
        fontSize: 12,
    },
});
