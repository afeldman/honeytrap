use serde::{Deserialize, Serialize};

/// LLM Provider Configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LLMProvider {
    DeepSeek {
        api_key: String,
        model: String,
    },
    OpenAI {
        api_key: String,
        model: String,
    },
    #[default]
    Disabled,
}

/// LLM Client für Verhaltensanalyse
pub struct LLMClient {
    provider: LLMProvider,
    client: reqwest::Client,
}

impl LLMClient {
    /// Neuer LLM Client
    pub fn new(provider: LLMProvider) -> Self {
        Self {
            provider,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    /// Verhaltensanalyse via LLM
    pub async fn analyze_behavior(
        &self,
        session_data: &SessionData,
    ) -> Result<BehaviorAnalysis, Box<dyn std::error::Error>> {
        match &self.provider {
            LLMProvider::DeepSeek { api_key, model } => {
                self.analyze_with_deepseek(api_key, model, session_data)
                    .await
            }
            LLMProvider::OpenAI { api_key, model } => {
                self.analyze_with_openai(api_key, model, session_data).await
            }
            LLMProvider::Disabled => Ok(BehaviorAnalysis::default()),
        }
    }

    /// DeepSeek API Call
    async fn analyze_with_deepseek(
        &self,
        api_key: &str,
        model: &str,
        session_data: &SessionData,
    ) -> Result<BehaviorAnalysis, Box<dyn std::error::Error>> {
        let prompt = self.build_analysis_prompt(session_data);

        let request = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a cybersecurity expert analyzing network traffic for malicious behavior. Respond with JSON only."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });

        tracing::debug!("🤖 Calling DeepSeek API...");

        let response = self
            .client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            tracing::error!("DeepSeek API error: {}", error_text);
            return Ok(BehaviorAnalysis::default());
        }

        let result: DeepSeekResponse = response.json().await?;
        self.parse_llm_response(&result.choices[0].message.content)
    }

    /// OpenAI API Call
    async fn analyze_with_openai(
        &self,
        api_key: &str,
        model: &str,
        session_data: &SessionData,
    ) -> Result<BehaviorAnalysis, Box<dyn std::error::Error>> {
        let prompt = self.build_analysis_prompt(session_data);

        let request = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a cybersecurity expert analyzing network traffic for malicious behavior. Respond with JSON only."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });

        tracing::debug!("🤖 Calling OpenAI API...");

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            tracing::error!("OpenAI API error: {}", error_text);
            return Ok(BehaviorAnalysis::default());
        }

        let result: OpenAIResponse = response.json().await?;
        self.parse_llm_response(&result.choices[0].message.content)
    }

    /// Prompt für LLM erstellen
    fn build_analysis_prompt(&self, session_data: &SessionData) -> String {
        format!(
            r#"Analyze this network session for malicious behavior:

Source IP: {}
Destination Port: {}
Connection Duration: {:.2}s
Bytes Sent: {}
Bytes Received: {}
Failed Login Attempts: {}
Commands Executed: {:?}
User Agent: {:?}
Request Pattern: {:?}

Provide analysis in JSON format:
{{
    "threat_level": "low|medium|high|critical",
    "threat_score": 0.0-1.0,
    "is_malicious": true|false,
    "attack_type": "port_scan|brute_force|sql_injection|xss|ddos|reconnaissance|credential_stuffing|none",
    "confidence": 0.0-1.0,
    "indicators": ["list of suspicious indicators"],
    "recommended_action": "block|honeypot|monitor|allow",
    "reasoning": "brief explanation"
}}"#,
            session_data.source_ip,
            session_data.destination_port,
            session_data.duration_secs,
            session_data.bytes_sent,
            session_data.bytes_received,
            session_data.failed_login_attempts,
            session_data.commands,
            session_data.user_agent,
            session_data.request_pattern,
        )
    }

    /// LLM Response parsen
    fn parse_llm_response(
        &self,
        content: &str,
    ) -> Result<BehaviorAnalysis, Box<dyn std::error::Error>> {
        let parsed: serde_json::Value = serde_json::from_str(content)?;

        Ok(BehaviorAnalysis {
            threat_level: parsed["threat_level"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            threat_score: parsed["threat_score"].as_f64().unwrap_or(0.0),
            is_malicious: parsed["is_malicious"].as_bool().unwrap_or(false),
            attack_type: parsed["attack_type"].as_str().unwrap_or("none").to_string(),
            confidence: parsed["confidence"].as_f64().unwrap_or(0.5),
            indicators: parsed["indicators"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            recommended_action: parsed["recommended_action"]
                .as_str()
                .unwrap_or("monitor")
                .to_string(),
            reasoning: parsed["reasoning"].as_str().unwrap_or("").to_string(),
        })
    }
}

/// Session-Daten für LLM-Analyse
#[derive(Debug, Clone, Serialize)]
pub struct SessionData {
    pub source_ip: String,
    pub destination_port: u16,
    pub duration_secs: f64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub failed_login_attempts: u32,
    pub commands: Vec<String>,
    pub user_agent: Option<String>,
    pub request_pattern: Option<String>,
}

/// Verhaltensanalyse vom LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysis {
    pub threat_level: String,
    pub threat_score: f64,
    pub is_malicious: bool,
    pub attack_type: String,
    pub confidence: f64,
    pub indicators: Vec<String>,
    pub recommended_action: String,
    pub reasoning: String,
}

impl Default for BehaviorAnalysis {
    fn default() -> Self {
        Self {
            threat_level: "unknown".to_string(),
            threat_score: 0.0,
            is_malicious: false,
            attack_type: "none".to_string(),
            confidence: 0.0,
            indicators: vec![],
            recommended_action: "monitor".to_string(),
            reasoning: "LLM disabled".to_string(),
        }
    }
}

// API Response Types
#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}
