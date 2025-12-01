//! HTTP Interaction Handler
//!
//! Erweiterte HTTP Honeypot-Interaktionen mit Fake Web Applications

use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// HTTP Method
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
}

/// HTTP Request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// HTTP Response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// HTTP Interaction Handler
pub struct HttpInteractionHandler {
    session_id: String,
    request_count: usize,
    login_attempts: Vec<(String, String)>,
}

impl HttpInteractionHandler {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            request_count: 0,
            login_attempts: Vec::new(),
        }
    }

    /// Handle HTTP request
    pub async fn handle_request(&mut self, request: HttpRequest) -> HttpResponse {
        self.request_count += 1;
        
        tracing::info!(
            "🌐 HTTP {:?} {} (Session: {})",
            request.method,
            request.path,
            self.session_id
        );

        // Detect suspicious patterns
        self.detect_attacks(&request);

        // Simulate processing delay
        sleep(Duration::from_millis(50)).await;

        // Route request
        match (request.method, request.path.as_str()) {
            (HttpMethod::GET, "/") => self.serve_homepage().await,
            (HttpMethod::GET, "/login") => self.serve_login_page().await,
            (HttpMethod::POST, "/login") => self.handle_login_post(request).await,
            (HttpMethod::GET, "/admin") => self.serve_admin_page().await,
            (HttpMethod::GET, "/api/config") => self.serve_fake_config().await,
            (HttpMethod::GET, path) if path.ends_with(".php") => self.handle_php_request(path).await,
            (HttpMethod::GET, path) if path.contains("..") => self.handle_directory_traversal(path).await,
            _ => self.serve_404().await,
        }
    }

    async fn serve_homepage(&self) -> HttpResponse {
        let body = r#"<!DOCTYPE html>
<html>
<head>
    <title>Corporate Portal</title>
</head>
<body>
    <h1>Welcome to Corporate Portal</h1>
    <p><a href="/login">Login</a></p>
    <p><a href="/admin">Admin Panel</a></p>
</body>
</html>"#;

        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: self.default_headers("text/html"),
            body: body.to_string(),
        }
    }

    async fn serve_login_page(&self) -> HttpResponse {
        let body = r#"<!DOCTYPE html>
<html>
<head>
    <title>Login - Corporate Portal</title>
</head>
<body>
    <h2>Login</h2>
    <form method="POST" action="/login">
        <input type="text" name="username" placeholder="Username"><br>
        <input type="password" name="password" placeholder="Password"><br>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#;

        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: self.default_headers("text/html"),
            body: body.to_string(),
        }
    }

    async fn handle_login_post(&mut self, request: HttpRequest) -> HttpResponse {
        if let Some(body) = &request.body {
            // Parse credentials
            let parts: Vec<&str> = body.split('&').collect();
            let mut username = String::new();
            let mut password = String::new();

            for part in parts {
                if let Some((key, value)) = part.split_once('=') {
                    match key {
                        "username" => username = urlencoding::decode(value).unwrap_or_default().to_string(),
                        "password" => password = urlencoding::decode(value).unwrap_or_default().to_string(),
                        _ => {}
                    }
                }
            }

            tracing::warn!(
                "🔑 HTTP Login attempt - User: {}, Pass: {} (Session: {})",
                username,
                password,
                self.session_id
            );

            self.login_attempts.push((username.clone(), password.clone()));

            // Simulate auth delay
            sleep(Duration::from_secs(1)).await;
        }

        let body = r#"<!DOCTYPE html>
<html>
<head>
    <title>Login Failed</title>
</head>
<body>
    <h2>Login Failed</h2>
    <p>Invalid credentials. <a href="/login">Try again</a></p>
</body>
</html>"#;

        HttpResponse {
            status: 401,
            status_text: "Unauthorized".to_string(),
            headers: self.default_headers("text/html"),
            body: body.to_string(),
        }
    }

    async fn serve_admin_page(&self) -> HttpResponse {
        let body = r#"<!DOCTYPE html>
<html>
<head>
    <title>Admin Panel</title>
</head>
<body>
    <h2>Access Denied</h2>
    <p>You must be logged in to access this page.</p>
    <p><a href="/login">Login</a></p>
</body>
</html>"#;

        HttpResponse {
            status: 403,
            status_text: "Forbidden".to_string(),
            headers: self.default_headers("text/html"),
            body: body.to_string(),
        }
    }

    async fn serve_fake_config(&self) -> HttpResponse {
        tracing::warn!("🚨 Config file access attempt (Session: {})", self.session_id);
        
        let body = r#"{
    "version": "1.0.0",
    "database": {
        "host": "localhost",
        "port": 3306,
        "name": "corporate_db"
    },
    "api_key": "sk-fake-key-12345",
    "admin_email": "admin@corporate.com"
}"#;

        HttpResponse {
            status: 200,
            status_text: "OK".to_string(),
            headers: self.default_headers("application/json"),
            body: body.to_string(),
        }
    }

    async fn handle_php_request(&self, path: &str) -> HttpResponse {
        tracing::warn!("🚨 PHP file access attempt: {} (Session: {})", path, self.session_id);
        
        HttpResponse {
            status: 404,
            status_text: "Not Found".to_string(),
            headers: self.default_headers("text/html"),
            body: "<html><body><h1>404 Not Found</h1></body></html>".to_string(),
        }
    }

    async fn handle_directory_traversal(&self, path: &str) -> HttpResponse {
        tracing::warn!("🚨 Directory traversal attempt: {} (Session: {})", path, self.session_id);
        
        HttpResponse {
            status: 403,
            status_text: "Forbidden".to_string(),
            headers: self.default_headers("text/html"),
            body: "<html><body><h1>403 Forbidden</h1></body></html>".to_string(),
        }
    }

    async fn serve_404(&self) -> HttpResponse {
        HttpResponse {
            status: 404,
            status_text: "Not Found".to_string(),
            headers: self.default_headers("text/html"),
            body: "<html><body><h1>404 Not Found</h1></body></html>".to_string(),
        }
    }

    fn default_headers(&self, content_type: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type.to_string());
        headers.insert("Server".to_string(), "Apache/2.4.41 (Ubuntu)".to_string());
        headers.insert("X-Powered-By".to_string(), "PHP/7.4.3".to_string());
        headers
    }

    fn detect_attacks(&self, request: &HttpRequest) {
        let path = &request.path;

        // SQL Injection patterns
        if path.contains("UNION") || path.contains("SELECT") || path.contains("'") {
            tracing::warn!("🚨 Possible SQL injection: {} (Session: {})", path, self.session_id);
        }

        // XSS patterns
        if path.contains("<script>") || path.contains("javascript:") {
            tracing::warn!("🚨 Possible XSS attack: {} (Session: {})", path, self.session_id);
        }

        // Command injection
        if path.contains(";") || path.contains("|") || path.contains("`") {
            tracing::warn!("🚨 Possible command injection: {} (Session: {})", path, self.session_id);
        }

        // LFI/RFI
        if path.contains("../") || path.contains("..\\") {
            tracing::warn!("🚨 Directory traversal detected: {} (Session: {})", path, self.session_id);
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> HttpStats {
        HttpStats {
            request_count: self.request_count,
            login_attempts: self.login_attempts.len(),
            captured_credentials: self.login_attempts.clone(),
        }
    }
}

/// HTTP Statistics
#[derive(Debug, Clone)]
pub struct HttpStats {
    pub request_count: usize,
    pub login_attempts: usize,
    pub captured_credentials: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_serve_homepage() {
        let mut handler = HttpInteractionHandler::new("test".to_string());
        let request = HttpRequest {
            method: HttpMethod::GET,
            path: "/".to_string(),
            headers: HashMap::new(),
            body: None,
        };
        
        let response = handler.handle_request(request).await;
        assert_eq!(response.status, 200);
        assert!(response.body.contains("Welcome"));
    }

    #[tokio::test]
    async fn test_login_attempt() {
        let mut handler = HttpInteractionHandler::new("test".to_string());
        let request = HttpRequest {
            method: HttpMethod::POST,
            path: "/login".to_string(),
            headers: HashMap::new(),
            body: Some("username=admin&password=test123".to_string()),
        };
        
        let response = handler.handle_request(request).await;
        assert_eq!(response.status, 401);
        assert_eq!(handler.get_stats().login_attempts, 1);
    }
}
