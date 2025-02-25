use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use rustyline::Editor;
use std::env;

const API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

async fn get_openrouter_answer(prompt: &str, api_key: &str, history: &mut Vec<Message>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    history.push(Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    });

    let request_body = ChatRequest {
        messages: history.clone(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);

    let response = client.post(API_URL)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;

    let chat_response: ChatResponse = response.json().await?;

    if let Some(choice) = chat_response.choices.first() {
        history.push(choice.message.clone());
        Ok(choice.message.content.clone())
    } else {
        Err("No content found in response".into())
    }
}

fn format_response(response: &str) -> String {
    let mut formatted = String::new();
    
    for line in response.lines() {
        if line.starts_with("### ") {
            formatted.push_str(&format!("\x1b[1;32m{}\x1b[0m\n", line));  // Bold green for headers
        } else {
            // Replace **text** with bold text
            let mut result = String::new();
            let mut is_bold = false;
            let mut last_pos = 0;
            
            for (i, _) in line.match_indices("**") {
                if is_bold {
                    result.push_str(&line[last_pos..i]);
                    result.push_str("\x1b[0m");  // Reset formatting
                } else {
                    result.push_str(&line[last_pos..i]);
                    result.push_str("\x1b[1m");  // Bold
                }
                is_bold = !is_bold;
                last_pos = i + 2;
            }
            result.push_str(&line[last_pos..]);
            formatted.push_str(&result);
            formatted.push('\n');
        }
    }
    formatted
}

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("Please set your API key in the OPENROUTER_API_KEY environment variable.");

    println!("\nChatGPT-like CLI Assistant");
    println!("========================");
    println!("Commands:");
    println!("  - Type 'exit' to quit");
    println!("  - Type 'clear context' to clear chat history");
    println!("========================\n");

    let mut history: Vec<Message> = Vec::new();
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(prompt) => {
                if prompt == "exit" {
                    break;
                }

                if prompt == "clear context" {
                    history.clear();
                    println!("Chat history cleared.\n");
                    continue;
                }

                if !prompt.trim().is_empty() {
                    match get_openrouter_answer(&prompt, &api_key, &mut history).await {
                        Ok(response) => {
                            println!("\n🤖 Assistant:");
                            print!("{}", format_response(&response));
                            println!();
                        }
                        Err(err) => {
                            println!("\n❌ Error: {}\n", err);
                        }
                    }
                }
            }
            Err(_) => {
                println!("\n");
                break;
            }
        }
    }
}