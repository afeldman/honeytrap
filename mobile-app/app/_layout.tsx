/**
 * @fileoverview Root layout for HoneyTrap mobile app.
 * Wraps the app with SafeAreaProvider for proper spacing on all devices.
 *
 * @example
 * // This is the root layout automatically loaded by Expo Router
 * // All routes are children of this component
 */

import { Slot } from "expo-router";
import { SafeAreaProvider } from "react-native-safe-area-context";

/**
 * Root layout component.
 *
 * @returns {JSX.Element} Root layout with SafeAreaProvider
 */
export default function Root() {
    return (
        <SafeAreaProvider>
            <Slot />
        </SafeAreaProvider>
    );
}
