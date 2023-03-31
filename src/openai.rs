use reqwest::blocking::Client;
use serde_json::json;

use crate::env::Environment;

pub fn eval(input: &str, client: &Client, env: &Environment) -> String {
  let prompt = input;

  let temperature = 0.2 + (0.8 - 0.2) * rand::random::<f64>();
  println!("Temperature: {}", temperature);

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