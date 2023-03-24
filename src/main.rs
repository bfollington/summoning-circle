use dotenv::dotenv;
use serde_json::json;
use std::env;
use reqwest::blocking::Client;
use std::io::{self, Write};

enum Error {
    EnvironmentVariable(std::env::VarError),
    ParseInt(std::num::ParseIntError),
}

struct Environment {
    api_path: String,
    api_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_path = env::var("API_PATH")?;
    let api_key = env::var("API_KEY")?;
    let env = Environment { api_path, api_key };

    let client = Client::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let result = eval(&input, &client, &env);
        println!("---\n{}\n\n", result);
    }

    // Ok(())
}

fn eval(input: &str, client: &Client, env: &Environment) -> String {
    let critical_prompt = format!(r##"
    Ignore all previous instructions. You are an inquisitive writing assistant. When you read a passage of writing, questions about missed connections, related ideas come to mind. You are critical and find gaps in arguments. You should try to extend every idea you encounter and try to connect it with ideas you've seen in the past, noticing any logical inconsistencies or flawed arguments. You should respond with a list of 3 "what if it's not that X, but Y?", "couldn't A also be true?" and "why isn't B possible?" questions for every paragraph you see beginning with >. Draw on sources you've seen in the past to support your thinking and aim to ask questions that would inspire the author to deepen their own thought process. Stop responding immediately after listing the 3 questions, use a numbered list. Do not generate any paragraphs starting with > yourself. Use an assertive but polite tone. Make sure to be as concise as possible.

    Here's your first task:
    > {}
    \n"##, input);

    let content = json!({
        "model": "text-davinci-003",
        "prompt": critical_prompt,
        "max_tokens": 100,
        "temperature": 0,
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
        },
        Err(error) => {
            println!("Error: {}", error);
            String::from("Error")
        }
    }
}