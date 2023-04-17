use reqwest::blocking::Client;

use crate::env::Environment;
use crate::openai::{Embedding, OpenAIError};
use crate::openai::{chatgpt, embedding};

pub struct Memory {
  pub subject: String,
  pub content: String,
  pub embedding: Embedding
}

pub struct Agent {
  pub base_prompt: String,
  pub memory_bank: Vec<Memory>
}

#[derive(Debug)]
pub enum AgentError {
  OpenAIError(crate::openai::OpenAIError)
}

impl From<OpenAIError> for AgentError {
  fn from(openai_error: OpenAIError) -> Self {
      AgentError::OpenAIError(openai_error)
  }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> Option<f64> {
  if a.len() != b.len() || a.is_empty() {
      return None;
  }

  let dot_product: f64 = a.iter().zip(b).map(|(a_elem, b_elem)| a_elem * b_elem).sum();
  let magnitude_a: f64 = a.iter().map(|a_elem| a_elem * a_elem).sum::<f64>().sqrt();
  let magnitude_b: f64 = b.iter().map(|b_elem| b_elem * b_elem).sum::<f64>().sqrt();

  if magnitude_a == 0.0 || magnitude_b == 0.0 {
      return None;
  }

  Some(dot_product / (magnitude_a * magnitude_b))
}

impl Agent {
  pub fn new(base_prompt: String) -> Agent {
      Agent {
          base_prompt,
          memory_bank: Vec::new()
      }
  }

  pub fn memorize(&mut self, subject: String, content: String, client: &Client, env: &Environment) -> Result<(), AgentError> {
      let embedding = embedding(content.as_str(), client, env)?;

      self.memory_bank.push(Memory {
          subject,
          content,
          embedding
      });

      Ok(())
  }

  pub fn recall(&self, embedding: Vec<f64>) -> Option<String> {
      let mut best_match: Option<String> = None;
      let mut best_match_similarity: f64 = 0.0;

      for memory in &self.memory_bank {
          let similarity = cosine_similarity(&memory.embedding, &embedding).unwrap();
          if similarity > best_match_similarity {
              best_match = Some(memory.content.clone());
              best_match_similarity = similarity;
          }
      }

      best_match
  }

  pub fn prompt(&self, input: &str, embedding: Vec<f64>) -> String {
      let mut prompt = self.base_prompt.clone();

      let relevant = self.recall(embedding);

      let message = match relevant {
          Some(memory) => format!("

          Message:
          {}
          
          Reminding me of:
          {}", input, memory),

          None => format!("
          
          Message:
          {}
          ", input)
      };

      println!("\n---\n{}\n---\n", message);

      prompt.push_str(&message);
      prompt
  }

  pub fn speak(&self, input: &str, client: &Client, env: &Environment) -> Result<String, AgentError> {
      let embedding = embedding(input, client, env)?;
      let prompt = self.prompt(input, embedding);
      let result = chatgpt(&prompt, &client, &env);

      Ok(result)
  }
}