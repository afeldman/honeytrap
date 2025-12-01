import { View, Text, StyleSheet } from "react-native";

export default function Sessions() {
    return (
        <View style={styles.container}>
            <Text style={styles.title}>Honeypot Sessions</Text>
            <Text style={styles.text}>Session details coming soon...</Text>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: "#111827",
        padding: 20,
    },
    title: {
        color: "#fff",
        fontSize: 24,
        fontWeight: "bold",
        marginBottom: 20,
    },
    text: {
        color: "#9ca3af",
        fontSize: 16,
    },
});
