use dotenv::dotenv;
use notes::{load_random_note, NoteError};
use reqwest::blocking::Client;
use std::env::{VarError};
use std::io::{self, Write};
use env::Environment;

mod env;
mod notes;
mod prompts;
mod subtext;
mod metaprompts;
mod openai;

#[derive(Debug)]
enum AppError {
    DotEnvError(dotenv::Error),
    EnvironmentError(VarError),
    NoteError(notes::NoteError),
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

fn print_menu() {
    print!(
        "
Commands:
[1] Load random note & analyse (critic)
[2] Load random note & analyse (actor)
[3] Load two random notes & compress
[4] Load random note & question
[5] Load random note & critique
[6] Free text input
");
}

fn main() -> Result<(), AppError> {
    dotenv()?;

    let env = Environment::from_env()?;
    let client = Client::new();

    loop {
        print_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        match trimmed_input {
            "1" => {
                println!("Random note analysis (critic)");
                // Implement your command 1 logic her
                let note = load_random_note()?;
                let prompt = metaprompts::critic(&note.content, &client, &env);
                println!("@{}\n\n", note.name);
                let result = openai::eval(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            "2" => {
                println!("Random note analysis (actor)");
                // Implement your command 1 logic her
                let note = load_random_note()?;
                let prompt = metaprompts::actor(&note.content, &client, &env);
                println!("@{}\n\n", note.name);
                let result = openai::eval(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            "3" => {
                println!("Random note combination");
                // Implement your command 1 logic her
                let note_a = load_random_note()?;
                let note_b = load_random_note()?;
                // combine note a and b content into one string
                let combined_notes = format!("{} {}", note_a.content, note_b.content);
                
                let prompt = prompts::compressor(&combined_notes);
                println!("@{}\n\n", note_a.name);
                println!("@{}\n\n", note_b.name);
                let result = openai::eval(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            "4" => {
                println!("Random questions from note");
                // Implement your command 1 logic her
                let note_a = load_random_note()?;
                
                let prompt = prompts::question_everything(&note_a.content);
                println!("@{}\n\n", note_a.name);
                let result = openai::eval(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            "5" => {
                println!("Random critique from note");
                // Implement your command 1 logic her
                let note_a = load_random_note()?;
                
                let prompt = prompts::critical_writing(&note_a.content);
                println!("@{}\n\n", note_a.name);
                let result = openai::eval(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            "6" => {
                print!("> ");
                io::stdout().flush().unwrap();

                let mut text_input = String::new();
                io::stdin().read_line(&mut text_input).unwrap();

                let prompt = metaprompts::critic(&text_input, &client, &env);
                let result = openai::eval(&prompt, &client, &env);
                println!("---\n{}\n\n", result);
            }
            "q" => break Ok(()),
            _ => println!("Invalid command. Please try again."),
        }
    }
}