/**
 * @fileoverview Application entry point for HoneyTrap Web UI.
 * Sets up React rendering with routing and global styles.
 *
 * @example
 * // This file is the entry point referenced in index.html:
 * // <script type="module" src="/src/main.tsx"></script>
 */

import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <BrowserRouter>
            <App />
        </BrowserRouter>
    </React.StrictMode>
);
