//! Advanced Honeypot Interactions
//!
//! Realistische Protokoll-Implementierungen und intelligente Response-Strategien

pub mod ssh_interaction;
pub mod http_interaction;
pub mod mysql_interaction;
pub mod command_parser;
pub mod fake_filesystem;
pub mod response_generator;

pub use ssh_interaction::SshInteractionHandler;
pub use http_interaction::HttpInteractionHandler;
pub use mysql_interaction::MysqlInteractionHandler;
pub use command_parser::{Command, CommandParser};
pub use fake_filesystem::{FakeFilesystem, FileEntry, FileType};
pub use response_generator::{ResponseGenerator, ResponseStrategy};
