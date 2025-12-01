/**
 * @fileoverview Main dashboard page with real-time metrics and visualizations.
 * Displays connection statistics, honeypot sessions, and system metrics.
 *
 * @example
 * // Dashboard updates every 5 seconds automatically
 * // Displays:
 * // - Connection stats (total, active, anomaly, CPU)
 * // - Pie chart of connection distribution
 * // - Bar chart of session types
 * // - Recent sessions table
 */

import { useEffect, useState } from "react";
import { honeytrapApi, DashboardData } from "@/api/client";
import { Activity, Shield, Database, Cpu } from "lucide-react";
import {
    LineChart,
    Line,
    BarChart,
    Bar,
    PieChart,
    Pie,
    Cell,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from "recharts";

/** Color palette for charts */
const COLORS = ["#10b981", "#f59e0b", "#ef4444"];

/**
 * Dashboard page component with real-time metrics.
 *
 * @returns {JSX.Element} Dashboard with stats, charts, and recent sessions
 *
 * @example
 * // Route configuration:
 * <Route path="dashboard" element={<Dashboard />} />
 */
export default function Dashboard() {
    const [data, setData] = useState<DashboardData | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const dashboardData = await honeytrapApi.getDashboardData();
                setData(dashboardData);
            } catch (error) {
                console.error("Failed to fetch dashboard data:", error);
            } finally {
                setLoading(false);
            }
        };

        fetchData();
        const interval = setInterval(fetchData, 5000);
        return () => clearInterval(interval);
    }, []);

    if (loading) {
        return (
            <div className="flex items-center justify-center h-96">
                <div className="text-gray-400">Loading...</div>
            </div>
        );
    }

    if (!data) {
        return (
            <div className="flex items-center justify-center h-96">
                <div className="text-red-400">
                    Failed to load dashboard data
                </div>
            </div>
        );
    }

    const pieData = [
        { name: "Normal", value: data.connections.normal },
        { name: "Anomaly", value: data.connections.anomaly },
        { name: "Blocked", value: data.connections.blocked },
    ];

    const sessionTypeData = data.honeypotSessions.reduce((acc, session) => {
        const existing = acc.find((item) => item.type === session.serviceType);
        if (existing) {
            existing.count += 1;
        } else {
            acc.push({ type: session.serviceType, count: 1 });
        }
        return acc;
    }, [] as { type: string; count: number }[]);

    return (
        <div className="space-y-6">
            <h1 className="text-3xl font-bold text-white">Dashboard</h1>

            {/* Stats Cards */}
            <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
                <StatCard
                    title="Total Connections"
                    value={data.connections.total}
                    icon={Activity}
                    color="blue"
                />
                <StatCard
                    title="Active Connections"
                    value={data.connections.active}
                    icon={Shield}
                    color="green"
                />
                <StatCard
                    title="Anomaly Detected"
                    value={data.connections.anomaly}
                    icon={Shield}
                    color="red"
                />
                <StatCard
                    title="CPU Usage"
                    value={`${data.systemMetrics.cpuUsage.toFixed(1)}%`}
                    icon={Cpu}
                    color="yellow"
                />
            </div>

            {/* Charts */}
            <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                {/* Connection Distribution */}
                <div className="bg-gray-800 rounded-lg p-6">
                    <h2 className="text-xl font-semibold text-white mb-4">
                        Connection Distribution
                    </h2>
                    <ResponsiveContainer width="100%" height={300}>
                        <PieChart>
                            <Pie
                                data={pieData}
                                cx="50%"
                                cy="50%"
                                labelLine={false}
                                label={(entry) =>
                                    `${entry.name}: ${entry.value}`
                                }
                                outerRadius={100}
                                fill="#8884d8"
                                dataKey="value"
                            >
                                {pieData.map((_, index) => (
                                    <Cell
                                        key={`cell-${index}`}
                                        fill={COLORS[index % COLORS.length]}
                                    />
                                ))}
                            </Pie>
                            <Tooltip />
                        </PieChart>
                    </ResponsiveContainer>
                </div>

                {/* Session Types */}
                <div className="bg-gray-800 rounded-lg p-6">
                    <h2 className="text-xl font-semibold text-white mb-4">
                        Honeypot Sessions by Type
                    </h2>
                    <ResponsiveContainer width="100%" height={300}>
                        <BarChart data={sessionTypeData}>
                            <CartesianGrid
                                strokeDasharray="3 3"
                                stroke="#374151"
                            />
                            <XAxis dataKey="type" stroke="#9ca3af" />
                            <YAxis stroke="#9ca3af" />
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: "#1f2937",
                                    border: "none",
                                }}
                            />
                            <Bar dataKey="count" fill="#10b981" />
                        </BarChart>
                    </ResponsiveContainer>
                </div>
            </div>

            {/* Recent Sessions */}
            <div className="bg-gray-800 rounded-lg p-6">
                <h2 className="text-xl font-semibold text-white mb-4">
                    Recent Sessions
                </h2>
                <div className="overflow-x-auto">
                    <table className="min-w-full divide-y divide-gray-700">
                        <thead>
                            <tr>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Type
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Source IP
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Duration
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Credentials
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Commands
                                </th>
                                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase">
                                    Status
                                </th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-gray-700">
                            {data.honeypotSessions
                                .slice(0, 10)
                                .map((session) => (
                                    <tr key={session.id}>
                                        <td className="px-6 py-4 whitespace-nowrap text-sm text-white">
                                            {session.serviceType.toUpperCase()}
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                            {session.sourceIp}
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                            {session.duration}s
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                            {session.credentialsCaptured}
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                                            {session.commandsExecuted}
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <span
                                                className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                                                    session.status === "active"
                                                        ? "bg-green-100 text-green-800"
                                                        : "bg-gray-100 text-gray-800"
                                                }`}
                                            >
                                                {session.status}
                                            </span>
                                        </td>
                                    </tr>
                                ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
}

interface StatCardProps {
    title: string;
    value: string | number;
    icon: React.ElementType;
    color: "blue" | "green" | "red" | "yellow";
}

function StatCard({ title, value, icon: Icon, color }: StatCardProps) {
    const colorClasses = {
        blue: "bg-blue-500",
        green: "bg-green-500",
        red: "bg-red-500",
        yellow: "bg-yellow-500",
    };

    return (
        <div className="bg-gray-800 rounded-lg p-6">
            <div className="flex items-center">
                <div className={`${colorClasses[color]} rounded-lg p-3`}>
                    <Icon className="w-6 h-6 text-white" />
                </div>
                <div className="ml-4">
                    <p className="text-sm font-medium text-gray-400">{title}</p>
                    <p className="text-2xl font-semibold text-white">{value}</p>
                </div>
            </div>
        </div>
    );
}
