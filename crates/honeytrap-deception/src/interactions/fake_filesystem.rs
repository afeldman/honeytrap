//! Fake Filesystem für Honeypot-Interaktionen
//!
//! Simuliert ein realistisches Linux-Dateisystem

use std::collections::HashMap;
use std::path::PathBuf;

/// File type
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Directory,
    File,
    Symlink,
}

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub file_type: FileType,
    pub permissions: String,
    pub size: u64,
    pub content: Option<String>,
    pub children: Vec<String>,
}

/// Fake Filesystem
pub struct FakeFilesystem {
    files: HashMap<PathBuf, FileEntry>,
    current_dir: PathBuf,
}

impl FakeFilesystem {
    /// Create new fake filesystem with realistic structure
    pub fn new() -> Self {
        let mut fs = Self {
            files: HashMap::new(),
            current_dir: PathBuf::from("/home/admin"),
        };

        fs.initialize_structure();
        fs
    }

    /// Initialize realistic filesystem structure
    fn initialize_structure(&mut self) {
        // Root directories
        self.add_dir("/", "drwxr-xr-x");
        self.add_dir("/home", "drwxr-xr-x");
        self.add_dir("/home/admin", "drwxr-xr-x");
        self.add_dir("/etc", "drwxr-xr-x");
        self.add_dir("/var", "drwxr-xr-x");
        self.add_dir("/tmp", "drwxrwxrwt");
        self.add_dir("/usr", "drwxr-xr-x");
        self.add_dir("/bin", "drwxr-xr-x");
        self.add_dir("/opt", "drwxr-xr-x");

        // Home directory files
        self.add_file("/home/admin/.bashrc", "-rw-r--r--", 220, Some("# .bashrc\nexport PS1='\\u@\\h:\\w\\$ '\n".to_string()));
        self.add_file("/home/admin/.bash_history", "-rw-------", 450, Some("ls\npwd\nwhoami\n".to_string()));
        self.add_file("/home/admin/.ssh", "drwx------", 0, None);
        
        // System files
        self.add_file("/etc/passwd", "-rw-r--r--", 1024, Some("root:x:0:0:root:/root:/bin/bash\nadmin:x:1000:1000::/home/admin:/bin/bash\n".to_string()));
        self.add_file("/etc/shadow", "-rw-------", 512, None); // No access (owner only)
        self.add_file("/etc/hosts", "-rw-r--r--", 156, Some("127.0.0.1 localhost\n".to_string()));
        
        // Var files
        self.add_dir("/var/log", "drwxr-xr-x");
        self.add_file("/var/log/syslog", "-rw-r-----", 4096, Some("Dec  1 10:00:01 server systemd[1]: Started session.\n".to_string()));
    }

    /// Add directory
    fn add_dir(&mut self, path: &str, permissions: &str) {
        let path = PathBuf::from(path);
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        self.files.insert(path, FileEntry {
            name,
            file_type: FileType::Directory,
            permissions: permissions.to_string(),
            size: 4096,
            content: None,
            children: Vec::new(),
        });
    }

    /// Add file
    fn add_file(&mut self, path: &str, permissions: &str, size: u64, content: Option<String>) {
        let path = PathBuf::from(path);
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        self.files.insert(path, FileEntry {
            name,
            file_type: FileType::File,
            permissions: permissions.to_string(),
            size,
            content,
            children: Vec::new(),
        });
    }

    /// List directory (ls)
    pub fn list_dir(&self, path: Option<&str>) -> Result<Vec<FileEntry>, String> {
        let target_path = if let Some(p) = path {
            self.resolve_path(p)
        } else {
            self.current_dir.clone()
        };

        // Find all files in this directory
        let mut entries = Vec::new();
        let target_str = target_path.to_string_lossy().to_string();

        for (path, entry) in &self.files {
            let path_str = path.to_string_lossy().to_string();
            if let Some(parent) = path.parent() {
                if parent.to_string_lossy() == target_str && path_str != target_str {
                    entries.push(entry.clone());
                }
            }
        }

        if entries.is_empty() && !self.files.contains_key(&target_path) {
            return Err(format!("ls: cannot access '{}': No such file or directory", target_str));
        }

        Ok(entries)
    }

    /// Get file content (cat)
    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let full_path = self.resolve_path(path);

        if let Some(entry) = self.files.get(&full_path) {
            match entry.file_type {
                FileType::File => {
                    // Simulate permission check (we're running as non-root user)
                    // Only check "other" permissions (last 3 chars)
                    let perms = &entry.permissions;
                    let other_read = perms.len() >= 10 && perms.chars().nth(7) == Some('r');
                    
                    if other_read {
                        Ok(entry.content.clone().unwrap_or_default())
                    } else {
                        Err(format!("cat: {}: Permission denied", path))
                    }
                }
                FileType::Directory => {
                    Err(format!("cat: {}: Is a directory", path))
                }
                FileType::Symlink => {
                    Ok(entry.content.clone().unwrap_or_default())
                }
            }
        } else {
            Err(format!("cat: {}: No such file or directory", path))
        }
    }

    /// Change directory (cd)
    pub fn change_dir(&mut self, path: &str) -> Result<(), String> {
        let new_path = self.resolve_path(path);

        if let Some(entry) = self.files.get(&new_path) {
            if entry.file_type == FileType::Directory {
                self.current_dir = new_path;
                Ok(())
            } else {
                Err(format!("cd: {}: Not a directory", path))
            }
        } else {
            Err(format!("cd: {}: No such file or directory", path))
        }
    }

    /// Get current directory (pwd)
    pub fn current_dir(&self) -> String {
        self.current_dir.to_string_lossy().to_string()
    }

    /// Resolve relative path to absolute
    fn resolve_path(&self, path: &str) -> PathBuf {
        if path.starts_with('/') {
            PathBuf::from(path)
        } else if path == "~" {
            PathBuf::from("/home/admin")
        } else if path.starts_with("~/") {
            PathBuf::from("/home/admin").join(&path[2..])
        } else if path == ".." {
            self.current_dir.parent().unwrap_or(&self.current_dir).to_path_buf()
        } else if path == "." {
            self.current_dir.clone()
        } else {
            self.current_dir.join(path)
        }
    }

    /// Check if file exists
    pub fn exists(&self, path: &str) -> bool {
        let full_path = self.resolve_path(path);
        self.files.contains_key(&full_path)
    }
}

impl Default for FakeFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_home_dir() {
        let fs = FakeFilesystem::new();
        let entries = fs.list_dir(Some("/home/admin")).unwrap();
        assert!(!entries.is_empty());
    }

    #[test]
    fn test_read_bashrc() {
        let fs = FakeFilesystem::new();
        let content = fs.read_file("/home/admin/.bashrc").unwrap();
        assert!(content.contains(".bashrc"));
    }

    #[test]
    fn test_change_directory() {
        let mut fs = FakeFilesystem::new();
        fs.change_dir("/etc").unwrap();
        assert_eq!(fs.current_dir(), "/etc");
    }

    #[test]
    fn test_pwd() {
        let fs = FakeFilesystem::new();
        assert_eq!(fs.current_dir(), "/home/admin");
    }

    #[test]
    fn test_permission_denied() {
        let fs = FakeFilesystem::new();
        let result = fs.read_file("/etc/shadow");
        assert!(result.is_err());
    }
}
