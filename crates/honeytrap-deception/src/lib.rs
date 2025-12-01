pub mod deception_system;
pub mod honeypots;

pub use deception_system::{DeceptionSystem, HoneypotConfig, InteractionLevel};
pub use honeypots::{Connection, Honeypot, HoneypotType, Session};
