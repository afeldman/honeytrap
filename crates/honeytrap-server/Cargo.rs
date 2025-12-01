[package]
name = "honeytrap-server"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net", "io-util"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
honeytrap-core = { path = "../honeytrap-core" }
honeytrap-policy = { path = "../honeytrap-policy" }
honeytrap-ai = { path = "../honeytrap-ai" }
honeytrap-deception = { path = "../honeytrap-deception" }
