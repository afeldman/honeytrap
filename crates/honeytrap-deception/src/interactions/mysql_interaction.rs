//! MySQL Interaction Handler
//!
//! Erweiterte MySQL Honeypot-Interaktionen

use std::time::Duration;
use tokio::time::sleep;

/// MySQL Protocol Version
const PROTOCOL_VERSION: u8 = 10;
const SERVER_VERSION: &str = "5.7.38-0ubuntu0.18.04.1";

/// MySQL Interaction Handler
pub struct MysqlInteractionHandler {
    session_id: String,
    authenticated: bool,
    username: Option<String>,
    database: Option<String>,
    query_count: usize,
}

impl MysqlInteractionHandler {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            authenticated: false,
            username: None,
            database: None,
            query_count: 0,
        }
    }

    /// Send MySQL handshake
    pub async fn send_handshake(&self) -> Vec<u8> {
        tracing::debug!("📤 Sending MySQL handshake (Session: {})", self.session_id);
        
        sleep(Duration::from_millis(100)).await;

        // Simplified MySQL handshake packet
        let mut packet = Vec::new();
        packet.push(PROTOCOL_VERSION);
        packet.extend_from_slice(SERVER_VERSION.as_bytes());
        packet.push(0); // null terminator
        
        packet
    }

    /// Handle authentication
    pub async fn authenticate(&mut self, username: &str, password: &str, database: Option<&str>) -> bool {
        tracing::info!(
            "🔑 MySQL Auth attempt - User: {}, DB: {:?} (Session: {})",
            username,
            database,
            self.session_id
        );

        if !password.is_empty() {
            tracing::warn!("📝 Captured MySQL credentials: {}:{}", username, password);
        }

        // Simulate auth delay
        sleep(Duration::from_secs(1)).await;

        self.authenticated = true;
        self.username = Some(username.to_string());
        self.database = database.map(|s| s.to_string());

        true // Always accept
    }

    /// Handle query
    pub async fn handle_query(&mut self, query: &str) -> MysqlResponse {
        self.query_count += 1;

        tracing::info!("💾 MySQL Query: {} (Session: {})", query, self.session_id);

        // Detect malicious patterns
        self.detect_malicious_query(query);

        // Simulate query execution
        sleep(Duration::from_millis(50)).await;

        let query_lower = query.to_lowercase();

        if query_lower.starts_with("show") {
            self.handle_show_query(&query_lower).await
        } else if query_lower.starts_with("select") {
            self.handle_select_query(&query_lower).await
        } else if query_lower.starts_with("use ") {
            self.handle_use_query(&query_lower).await
        } else if query_lower.starts_with("insert") || query_lower.starts_with("update") || query_lower.starts_with("delete") {
            tracing::warn!("🚨 Data modification attempt: {} (Session: {})", query, self.session_id);
            MysqlResponse::Ok { affected_rows: 0 }
        } else {
            MysqlResponse::Error {
                code: 1064,
                message: "You have an error in your SQL syntax".to_string(),
            }
        }
    }

    async fn handle_show_query(&self, query: &str) -> MysqlResponse {
        if query.contains("databases") {
            MysqlResponse::ResultSet {
                columns: vec!["Database".to_string()],
                rows: vec![
                    vec!["information_schema".to_string()],
                    vec!["mysql".to_string()],
                    vec!["corporate_db".to_string()],
                    vec!["test".to_string()],
                ],
            }
        } else if query.contains("tables") {
            MysqlResponse::ResultSet {
                columns: vec!["Tables_in_corporate_db".to_string()],
                rows: vec![
                    vec!["users".to_string()],
                    vec!["sessions".to_string()],
                    vec!["logs".to_string()],
                ],
            }
        } else if query.contains("variables") {
            MysqlResponse::ResultSet {
                columns: vec!["Variable_name".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["version".to_string(), SERVER_VERSION.to_string()],
                    vec!["datadir".to_string(), "/var/lib/mysql/".to_string()],
                ],
            }
        } else {
            MysqlResponse::Ok { affected_rows: 0 }
        }
    }

    async fn handle_select_query(&self, query: &str) -> MysqlResponse {
        if query.contains("version()") {
            MysqlResponse::ResultSet {
                columns: vec!["version()".to_string()],
                rows: vec![vec![SERVER_VERSION.to_string()]],
            }
        } else if query.contains("user()") {
            let user = self.username.as_ref().map(|u| format!("{}@localhost", u)).unwrap_or_else(|| "guest@localhost".to_string());
            MysqlResponse::ResultSet {
                columns: vec!["user()".to_string()],
                rows: vec![vec![user]],
            }
        } else if query.contains("database()") {
            let db = self.database.as_ref().map(|d| d.clone()).unwrap_or_else(|| "NULL".to_string());
            MysqlResponse::ResultSet {
                columns: vec!["database()".to_string()],
                rows: vec![vec![db]],
            }
        } else if query.contains("from") {
            // Generic SELECT FROM query
            tracing::warn!("🔍 Data extraction attempt: {} (Session: {})", query, self.session_id);
            MysqlResponse::ResultSet {
                columns: vec!["id".to_string(), "name".to_string()],
                rows: vec![
                    vec!["1".to_string(), "sample_data".to_string()],
                ],
            }
        } else {
            MysqlResponse::ResultSet {
                columns: vec!["result".to_string()],
                rows: vec![vec!["1".to_string()]],
            }
        }
    }

    async fn handle_use_query(&mut self, query: &str) -> MysqlResponse {
        if let Some(db_name) = query.strip_prefix("use ").map(|s| s.trim()) {
            self.database = Some(db_name.to_string());
            tracing::info!("📂 Database changed to: {} (Session: {})", db_name, self.session_id);
            MysqlResponse::Ok { affected_rows: 0 }
        } else {
            MysqlResponse::Error {
                code: 1049,
                message: "Unknown database".to_string(),
            }
        }
    }

    fn detect_malicious_query(&self, query: &str) {
        let query_lower = query.to_lowercase();

        // SQL Injection patterns
        if query_lower.contains("union") && query_lower.contains("select") {
            tracing::warn!("🚨 SQL Injection (UNION) detected: {} (Session: {})", query, self.session_id);
        }

        if query_lower.contains("--") || query_lower.contains("#") {
            tracing::warn!("🚨 SQL comment injection detected: {} (Session: {})", query, self.session_id);
        }

        if query_lower.contains("sleep(") || query_lower.contains("benchmark(") {
            tracing::warn!("🚨 Time-based SQL injection detected: {} (Session: {})", query, self.session_id);
        }

        // Data exfiltration
        if query_lower.contains("into outfile") || query_lower.contains("into dumpfile") {
            tracing::warn!("🚨 File write attempt detected: {} (Session: {})", query, self.session_id);
        }

        // Privilege escalation
        if query_lower.contains("grant") || query_lower.contains("create user") {
            tracing::warn!("🚨 Privilege escalation attempt: {} (Session: {})", query, self.session_id);
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> MysqlStats {
        MysqlStats {
            query_count: self.query_count,
            authenticated: self.authenticated,
            username: self.username.clone(),
            database: self.database.clone(),
        }
    }
}

/// MySQL Response
#[derive(Debug, Clone)]
pub enum MysqlResponse {
    Ok { affected_rows: u64 },
    Error { code: u16, message: String },
    ResultSet { columns: Vec<String>, rows: Vec<Vec<String>> },
}

/// MySQL Statistics
#[derive(Debug, Clone)]
pub struct MysqlStats {
    pub query_count: usize,
    pub authenticated: bool,
    pub username: Option<String>,
    pub database: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mysql_handler_creation() {
        let handler = MysqlInteractionHandler::new("test".to_string());
        assert!(!handler.authenticated);
    }

    #[tokio::test]
    async fn test_authentication() {
        let mut handler = MysqlInteractionHandler::new("test".to_string());
        let result = handler.authenticate("root", "password", Some("mysql")).await;
        assert!(result);
        assert!(handler.authenticated);
    }

    #[tokio::test]
    async fn test_show_databases() {
        let mut handler = MysqlInteractionHandler::new("test".to_string());
        handler.authenticate("test", "test", None).await;
        
        let response = handler.handle_query("SHOW DATABASES").await;
        if let MysqlResponse::ResultSet { rows, .. } = response {
            assert!(!rows.is_empty());
        } else {
            panic!("Expected ResultSet");
        }
    }

    #[tokio::test]
    async fn test_sql_injection_detection() {
        let mut handler = MysqlInteractionHandler::new("test".to_string());
        handler.authenticate("test", "test", None).await;
        
        let _response = handler.handle_query("SELECT * FROM users UNION SELECT NULL,NULL,NULL--").await;
        // Should log warning (checked in logs)
    }
}
