import { Routes, Route, Navigate } from "react-router-dom";
import Layout from "./components/Layout";
import Dashboard from "./pages/Dashboard";
import Connections from "./pages/Connections";
import Sessions from "./pages/Sessions";
import MLMetrics from "./pages/MLMetrics";

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
