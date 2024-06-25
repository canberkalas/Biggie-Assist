use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct OpenAIRequest {
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

pub async fn handle_transcript(transcript: String) {
    println!("Transcript: {}", transcript);

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = Client::new();
    let request = OpenAIRequest {
        prompt: format!("Respond to this call transcript: {}", transcript),
        max_tokens: 150,
    };

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    let response_body: OpenAIResponse = response.json().await.expect("Failed to parse response");
    println!("Response: {}", response_body.choices[0].text);
}
