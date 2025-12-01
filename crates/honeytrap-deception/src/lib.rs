pub mod honeypots;
pub mod deception_system;

pub use deception_system::{DeceptionSystem, HoneypotConfig, InteractionLevel};
pub use honeypots::{Honeypot, HoneypotType, Connection, Session};
