use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub system_prompt: String,
    pub port: u16,
    pub data_file: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Loading API KEY from .env otherwise error
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY env variable is not set".to_string())?;

        // Load model
        let model = env::var("MODEL").unwrap_or_else(|_| "gemini-3.5-flash-lite".to_string());

        // Read max max_tokens
        let max_tokens = env::var("MAX_TOKENS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1024);

        let system_prompt = env::var("SYSTEM_PROMPT").unwrap_or_else(|_| {
            "You are Cody. A friendly, helpful assistant. Keep answers short and concise"
                .to_string()
        });

        let port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);

        let data_file =
            env::var("DATA_FILE").unwrap_or_else(|_| "data/conversations.json".to_string());

        Ok(Self {
            api_key,
            model,
            max_tokens,
            system_prompt,
            port,
            data_file,
        })
    }
}
