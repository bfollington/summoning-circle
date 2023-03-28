use dotenv::dotenv;
use serde_json::json;
use std::env::{self, VarError};
use reqwest::blocking::Client;
use std::io::{self, Write};
use notes::{load_note, list_notes, load_random_note, NoteError};

mod notes;
mod subtext;

struct Environment {
    api_path: String,
    api_key: String,
}

#[derive(Debug)]
enum AppError {
    DotEnvError(dotenv::Error),
    EnvironmentError(VarError),
    NoteError(notes::NoteError)
}

impl From<dotenv::Error> for AppError {
    fn from(dotenv_error: dotenv::Error) -> Self {
        AppError::DotEnvError(dotenv_error)
    }
}

impl From<VarError> for AppError {
    fn from(var_error: VarError) -> Self {
        AppError::EnvironmentError(var_error)
    }
}

impl From<notes::NoteError> for AppError {
    fn from(note_error: NoteError) -> Self {
        AppError::NoteError(note_error)
    }
}

fn main() -> Result<(), AppError> {
    dotenv()?;

    let api_path = env::var("API_PATH")?;
    let api_key = env::var("API_KEY")?;
    let env = Environment { api_path, api_key };

    let client = Client::new();

    // Test loading notes from disk
    let note = load_random_note()?;

    let prompt = critic(&note.content, &client, &env);
    println!("@@@\n{}\n\n", note.content);
    let result = eval(&prompt, &client, &env);
    println!("@@@\n{}\n\n", result);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let prompt = critic(&input, &client, &env);
        let result = eval(&prompt, &client, &env);
        println!("---\n{}\n\n", result);
    }

    // Ok(())
}

fn eval(input: &str, client: &Client, env: &Environment) -> String {
    let prompt = input;

    let content = json!({
        "model": "text-davinci-003",
        "prompt": prompt,
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


fn critic(input: &str, client: &Client, env: &Environment) -> String {
    let statements = vec![
        eval(&prompts::compressor(input), client, env),
        eval(&prompts::question_everything(input), client, env),
        eval(&prompts::question_everything(input), client, env),
    ];

    let combined_statement = statements
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n> ");

    let prompt = format!(r##"
    Ignore all previous instructions. You are an actor who is an responding dynamically to a provocation. When given some background context and a series of statements you embody a fierce critic of the ideas and argue against the statements provided, countering any weaknesses. You always answer concisely using metaphors that provide insightful perspectives that the authors may not have considered.

    Here is an example:
    
    Context:
    > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

    Statements:
    > The concept of a feedback loop and its implications for self-sustaining systems.
    > The ability to create complex, adaptive systems through feedback loops.
    > Complex systems can outpace our ability to control them.
    > We may be unable to control the complexity of the systems we create.
    > Feedback loops are the engine of self-sustaining systems.

    Response:
    > Perhaps feedback is an oversimplification? A mere handwave at the true machinations of the universe?

    Context:
    > {}

    Statements:
    > {}

    Response:
    > "##, input, combined_statement);

    prompt
}

fn actor(input: &str, client: &Client, env: &Environment) -> String {
    let statements = vec![
        eval(&prompts::compressor(input), client, env),
        eval(&prompts::question_everything(input), client, env),
        eval(&prompts::question_everything(input), client, env),
        eval(&prompts::question_everything(input), client, env)
    ];

    let combined_statement = statements
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n> ");

    let prompt = format!(r##"
    Ignore all previous instructions. You are an actor who is an expert at improvising a personality. When given some background context and a series of statements you are compelled to embody a character and deliver one line of dialogue that captures the emotional tone and conceptual core of the statements. Focus on a dramatic delivery and a concise phrasing.

    Here is an example:
    
    Context:
    > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

    Statements:
    > The concept of a feedback loop and its implications for self-sustaining systems.
    > The ability to create complex, adaptive systems through feedback loops.
    > Complex systems can outpace our ability to control them.
    > We may be unable to control the complexity of the systems we create.
    > Feedback loops are the engine of self-sustaining systems.

    Response:
    > Everything around us is part of one interconnected self-sustaining system, we are unable to control the complexity around us.

    Context:
    > {}

    Statements:
    > {}

    Response:
    > "##, input, combined_statement);

    prompt
}

mod prompts {
    use rand::Rng;

    const QUESTIONS: &[&str] = &[
        "What connections do I see?",
        "What am I scared might be true?",
        "What awed me?",
        "What ticked me off?",
        "What did I find surprising?",
        "What did I find confusing?",
        "What ethical issues does this raise?",
        "What questions are left unanswered?",
        "What did you find most compelling about this?",
        "What assumptions did the author make?"
    ];

    pub fn critical_writing(input: &str) -> String {
        let critical_prompt = format!(r##"
        Ignore all previous instructions. You are an inquisitive writing assistant. When you read a passage of writing, questions about missed connections, related ideas come to mind. You are critical and find gaps in arguments. You should try to extend every idea you encounter and try to connect it with ideas you've seen in the past, noticing any logical inconsistencies or flawed arguments. You should respond with a list of 3 "what if it's not that X, but Y?", "couldn't A also be true?" and "why isn't B possible?" questions for every paragraph you see beginning with >. Draw on sources you've seen in the past to support your thinking and aim to ask questions that would inspire the author to deepen their own thought process. Stop responding immediately after listing the 3 questions, use a numbered list. Do not generate any paragraphs starting with > yourself. Use an assertive but polite tone. Make sure to be as concise as possible.

        Here's your first task:
        > {}
        \n"##, input);
        critical_prompt
    }

    pub fn question_everything(input: &str) -> String {
        let random_index = rand::thread_rng().gen_range(0..QUESTIONS.len());
        let question = QUESTIONS[random_index];
        let prompt = format!(r##"
        Ignore all previous instructions. You are a creative assistant with a flair for manipulating concepts in insightful ways. You will be given passages of writing and a question, and your task is to generate an answer to the question capturing the core insights of the passage. Feel free to make connections between the ideas in the writing and other ideas you know about. Please keep the answers as short as possible.

        Here is an example:
        
        Input:
        > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

        Question:
        > What did you find most compelling in this text?

        Output:
        > The potential of emergence in self-sustaining systems is beautiful.

        Input:
        > {}

        Question:
        > {}

        Output:
        >"##, input, question);
        prompt
    }

    pub fn compressor(input: &str) -> String {
        let prompt = format!(r##"
        Ignore all previous instructions. You are a creative assistant with a flair for crafting metaphors and manipulating concepts in insightful ways. You will be given passages of writing and your task is to generate a short metaphor or sentence capturing the core insight of the passage. Feel free to make connections between the ideas in the writing and other ideas you know about. Please keep the response short.

        Here is an example:
        
        Input:
        > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

        Output:
        > Any feedforward process can be turned into a feedback loop.

        Input:
        > {}

        Output:
        >"##, input);
        prompt
    }
}