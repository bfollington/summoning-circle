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

enum Command {
    Critic,
    Actor,
    FourActor,
    Compress,
    Question,
    Critique,
    Connect,
    FreeText,
    Conversation,
    Quit
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Critic => "Load random note & analyse (critic) (ChatGPT)",
            Command::Actor => "Load random note & analyse (actor) (ChatGPT)",
            Command::FourActor => "Load 4 random notes & analyse (actor) (ChatGPT)",
            Command::Compress => "Load two random notes & compress",
            Command::Question => "Load random note & question",
            Command::Critique => "Load random note & critique",
            Command::Connect => "Load random note & connect to random notes",
            Command::FreeText => "Free text input",
            Command::Conversation => "Conversation between geists",
            Command::Quit => "Quit"
        }.to_string()
    }
}

const MENU: [Command; 10] = [
    Command::Critic,
    Command::Actor,
    Command::FourActor,
    Command::Compress,
    Command::Question,
    Command::Critique,
    Command::Connect,
    Command::FreeText,
    Command::Conversation,
    Command::Quit
];

fn print_menu() {
    let menu_string = MENU.iter().enumerate().map(|(i, command)| format!("[{}] {}", i + 1, command.to_string())).collect::<Vec<String>>().join("\n");

    print!("Commands:\n{}\n", menu_string);
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
        let index = match trimmed_input.parse::<usize>() {
            Ok(i) => i - 1,
            Err(_) => {
                println!("Invalid command. Please try again.");
                continue;
            }
        };
       
        let trimmed_input = input.trim();

        let command = match MENU.get(index) {
            Some(c) => c,
            None => {
                println!("Invalid command. Please try again.");
                continue;
            }
        };

        match command {
            Command::Critic => {
                println!("Random note analysis (critic)");
                let note = load_random_note()?;
                let prompt = metaprompts::critic(&note.content, &client, &env);
                println!("@{}\n\n", note.name);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::Actor => {
                println!("Random note analysis (actor)");
                let note = load_random_note()?;
                let prompt = metaprompts::actor(&note.content, &client, &env);
                println!("@{}\n\n", note.name);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::FourActor => {
                println!("Random 4 note analysis (actor)");
                let (note, note_a, note_b, note_c) = (load_random_note()?, load_random_note()?, load_random_note()?, load_random_note()?);
                let prompt = metaprompts::giga_actor(&note.content, &note_a.content, &note_b.content, &note_c.content, &client, &env);
                println!("@{}\n\n", note.name);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::Compress => {
                println!("Random note combination");
                let note_a = load_random_note()?;
                let note_b = load_random_note()?;
                // combine note a and b content into one string
                let combined_notes = format!("{} {}", note_a.content, note_b.content);
                
                let prompt = prompts::compressor(&combined_notes);
                println!("@{}\n\n", note_a.name);
                println!("@{}\n\n", note_b.name);
                let result = openai::gpt3(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::Question => {
                println!("Random questions from note");
                let note_a = load_random_note()?;
                
                let prompt = prompts::question_everything(&note_a.content);
                println!("@{}\n\n", note_a.name);
                let result = openai::gpt3(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::Critique => {
                println!("Random critique from note");
                let note_a = load_random_note()?;
                
                let prompt = prompts::critical_writing(&note_a.content);
                println!("@{}\n\n", note_a.name);
                let result = openai::gpt3(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::Connect => {
                println!("Random note with connections to random notes");
                let note_base = load_random_note()?;

                let note_a = load_random_note()?;
                let note_b = load_random_note()?;
                let note_c = load_random_note()?;
                
                let prompt = prompts::connections(&note_base.content, &note_a.content, &note_b.content, &note_c.content);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("@@@\n{}\n\n", result);
            }
            Command::FreeText => {
                print!("> ");
                io::stdout().flush().unwrap();

                let mut text_input = String::new();
                io::stdin().read_line(&mut text_input).unwrap();

                let prompt = metaprompts::critic(&text_input, &client, &env);
                let result = openai::gpt3(&prompt, &client, &env);
                println!("---\n{}\n\n", result);
            },
            Command::Conversation => {
                conversation(&client, &env)?
            }
            Command::Quit => break Ok(())
        }
    }
}

fn wait_for_enter() {
    print!("Press enter to continue...");
    io::stdout().flush().unwrap();
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).unwrap();
}

fn conversation(client: &Client, env: &Environment) -> Result<(), AppError> {
    let note_base = load_random_note()?;

    print!("> ");
    println!("{}", note_base.name);
    println!("{}", note_base.content);
    io::stdout().flush().unwrap();

    let mut results = vec![note_base.content.clone()];

    loop {
        let input = results.join("\n-\nREPLY:\n");

        let random_int = rand::random::<u8>() % 7;
        match random_int {
            0 | 4 | 5 | 6 => {
                let prompt = prompts::chatter(&input);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("---\n{}\n\n", result);

                results.push(result);
            },
            1 => {
                let prompt = metaprompts::critic(&input, &client, &env);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("---\n{}\n\n", result);

                results.push(result);
            },
            2 => {
                let prompt = metaprompts::actor(&input, &client, &env);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("---\n{}\n\n", result);

                results.push(result);
            },
            3 => {
                let prompt = prompts::compressor(&input);
                let result = openai::chatgpt(&prompt, &client, &env);
                println!("---\n{}\n\n", result);

                results.push(result);
            },
            _ => unreachable!(),
        }

        // clear memory, breaks out of same patterns
        if results.len() > 3 {
            results.remove(0);
        }

        wait_for_enter();
    }
}