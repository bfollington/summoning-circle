use crate::openai::{gpt3};
use crate::prompts;
use crate::env;
use reqwest::blocking::Client;

pub fn critic(input: &str, client: &Client, env: &env::Environment) -> String {
  let statements = vec![
      gpt3(&prompts::compressor(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
  ];

  let combined_statement = statements
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<&str>>()
      .join("\n> ");

  let prompt = format!(
      r##"
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
  > "##,
      input, combined_statement
  );

  prompt
}

pub fn actor(input: &str, client: &Client, env: &env::Environment) -> String {
  let statements = vec![
      gpt3(&prompts::compressor(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
  ];

  let combined_statement = statements
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<&str>>()
      .join("\n> ");

  let prompt = format!(
      r##"
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
  > "##,
      input, combined_statement
  );

  prompt
}

pub fn giga_actor(input: &str, note_a: &str, note_b: &str, note_c: &str, client: &Client, env: &env::Environment) -> String {
  let statements = vec![
      gpt3(&prompts::compressor(input), client, env),
      gpt3(&prompts::question_everything(input), client, env),
      gpt3(&prompts::question_everything(note_a), client, env),
      gpt3(&prompts::question_everything(note_b), client, env),
      gpt3(&prompts::question_everything(note_c), client, env),
      gpt3(&prompts::connections(input, note_a, note_b, note_c), client, env),
  ];

  let combined_statement = statements
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<&str>>()
      .join("\n> ");

  let prompt = format!(
      r##"
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
  ---
  > {}
  ---
  > {}
  ---
  > {}

  Statements:
  > {}

  Response:
  > "##,
      input, note_a, note_b, note_c, combined_statement
  );

  prompt
}
