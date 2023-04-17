use reqwest::blocking::Client;
use serde_json::json;

use crate::env::Environment;

pub type Embedding = Vec<f64>;

pub enum OpenAIError {
    Error(String)
}

pub fn embedding(input: &str, client: &Client, env: &Environment) -> Result<Embedding, OpenAIError> {
    let content = json!({
        "model": "text-embedding-ada-002",
        "input": input
    });

    let response = client
        .post(format!("{}/embeddings", env.api_path))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .json(&content)
        .send();

    match response {
        Ok(response) => {
            let text = response.text().unwrap();
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();

            match json["error"].as_object() {
                Some(error) => {
                    println!("Error: {}", error["message"]);
                    return Err(OpenAIError::Error(error["message"].to_string()));
                }
                None => {}
            }

            let data = json["data"].as_array().unwrap();
            let result = data[0].as_object().unwrap();
            let embedding = result["embedding"].as_array().unwrap();
            Ok(embedding.iter().map(|x| x.as_f64().unwrap()).collect::<Embedding>())
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(OpenAIError::Error(error.to_string()))
        }
    }
}

pub fn gpt3(input: &str, client: &Client, env: &Environment) -> String {
    let prompt = input;

    let temperature = 0.2 + (0.6 - 0.2) * rand::random::<f64>();
    println!("GPT-3 Temperature: {}", temperature);

    let content = json!({
        "model": "text-davinci-003",
        "prompt": prompt,
        "max_tokens": 100,
        "temperature": temperature,
        "top_p": 1,
        "n": 1,
        "stream": false,
    });

    let response = client
        .post(format!("{}/completions", env.api_path))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .json(&content)
        .send();

    match response {
        Ok(response) => {
            let text = response.text().unwrap();
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();

            match json["error"].as_object() {
                Some(error) => {
                    println!("Error: {}", error["message"]);
                    return String::from("Error");
                }
                None => {}
            }

            let choices = json["choices"].as_array().unwrap();
            let choice = choices[0].as_object().unwrap();
            let text = choice["text"].as_str().unwrap();
            text.to_string()
        }
        Err(error) => {
            println!("Error: {}", error);
            String::from("Error")
        }
    }
}

pub fn chatgpt(input: &str, client: &Client, env: &Environment) -> String {
    let prompt = input;

    let temperature = 0.2 + (0.6 - 0.2) * rand::random::<f64>();
    println!("ChatGPT Temperature: {}", temperature);

    let content = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
          {"role": "user", "content": prompt}
        ],
        "max_tokens": 100,
        "temperature": temperature,
        "top_p": 1,
        "n": 1,
        "stream": false,
    });

    let response = client
        .post(format!("{}/chat/completions", env.api_path))
        .header("Authorization", format!("Bearer {}", env.api_key))
        .json(&content)
        .send();

    match response {
        Ok(response) => {
            let text = response.text().unwrap();
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();

            match json["error"].as_object() {
                Some(error) => {
                    println!("Error: {}", error["message"]);
                    return String::from("Error");
                }
                None => {}
            }

            let choices = json["choices"].as_array().unwrap();
            let choice = choices[0].as_object().unwrap();
            let message = choice["message"].as_object().unwrap();
            let text = message["content"].as_str().unwrap();
            text.to_string()
        }
        Err(error) => {
            println!("Error: {}", error);
            String::from("Error")
        }
    }
}
