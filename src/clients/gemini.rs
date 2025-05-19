use std::env;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn send_to_gemini(prompt: &str) -> Result<String, anyhow::Error> {
    let api_key = env::var("GEMINI_API_KEY")?;
    let client = Client::new();

    #[derive(Serialize)]
    struct GeminiRequest {
        contents: Vec<Content>,
        generation_config: Option<GenerationConfig>,
    }

    #[derive(Serialize)]
    struct GenerationConfig {
        temperature: f32,
    }

    #[derive(Serialize)]
    struct Content {
        parts: Vec<Part>,
        role: String,
    }

    #[derive(Serialize)]
    struct Part {
        text: String,
    }

    #[derive(Deserialize)]
    struct GeminiResponse {
        candidates: Vec<Candidate>,
    }

    #[derive(Deserialize)]
    struct Candidate {
        content: ContentResponse,
    }

    #[derive(Deserialize)]
    struct ContentResponse {
        parts: Vec<PartResponse>,
    }

    #[derive(Deserialize)]
    struct PartResponse {
        text: String,
    }

    let initial_prompt = env::var("GEMINI_INITIAL_PROMPT")?;

    let req_body = GeminiRequest {
        contents: vec![Content {
            role: "user".into(),
            parts: vec![Part {
                text: format!("{}. Answer this: {}", initial_prompt, prompt),
            }],
        }],
        generation_config: Some(GenerationConfig { temperature: 0.7 }),
    };

    let res = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent")
        .query(&[("key", api_key)])
        .json(&req_body)
        .send()
        .await?
        .json::<GeminiResponse>()
        .await?;

    let output = res
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or("Нічого не зрозумів 🤔".to_string());

    Ok(output)
}
