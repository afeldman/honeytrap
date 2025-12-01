/**
 * @fileoverview Tab navigation layout for HoneyTrap mobile app.
 * Defines bottom tab navigation with 4 screens: Dashboard, Connections, Sessions, ML.
 *
 * @example
 * // Tabs automatically handle navigation between screens:
 * // - Dashboard (index) - Main dashboard with stats
 * // - Connections - Connection monitoring
 * // - Sessions - Honeypot session viewer
 * // - ML - Machine learning metrics
 */

import { Tabs } from "expo-router";
import { Ionicons } from "@expo/vector-icons";

/**
 * Tab navigation layout component.
 *
 * @returns {JSX.Element} Tab navigator with 4 screens
 */
export default function TabLayout() {
    return (
        <Tabs
            screenOptions={{
                headerStyle: {
                    backgroundColor: "#111827",
                },
                headerTintColor: "#fff",
                tabBarStyle: {
                    backgroundColor: "#1f2937",
                    borderTopColor: "#374151",
                },
                tabBarActiveTintColor: "#10b981",
                tabBarInactiveTintColor: "#9ca3af",
            }}
        >
            <Tabs.Screen
                name="index"
                options={{
                    title: "Dashboard",
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons
                            name="speedometer"
                            size={size}
                            color={color}
                        />
                    ),
                }}
            />
            <Tabs.Screen
                name="connections"
                options={{
                    title: "Connections",
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="shield" size={size} color={color} />
                    ),
                }}
            />
            <Tabs.Screen
                name="sessions"
                options={{
                    title: "Sessions",
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="server" size={size} color={color} />
                    ),
                }}
            />
            <Tabs.Screen
                name="ml"
                options={{
                    title: "ML Metrics",
                    tabBarIcon: ({ color, size }) => (
                        <Ionicons name="analytics" size={size} color={color} />
                    ),
                }}
            />
        </Tabs>
    );
}
