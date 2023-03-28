use std::{env, path::Path};
use crate::subtext::Subtext;

#[derive(Debug)]
pub enum NoteError {
  IOError(std::io::Error)
}

pub fn load_note(name: String) -> Result<Subtext, NoteError> {
    let current_dir = env::current_dir().unwrap();
    let full_file_path = current_dir.join(Path::new("notes")).join(Path::new(&name));

    return Subtext::from_file(full_file_path)
      .map_err(|e| NoteError::IOError(e));
}

pub fn list_notes() -> Vec<String> {
  let current_dir = env::current_dir().unwrap();
  let notes_dir = current_dir.join(Path::new("notes"));

  std::fs::read_dir(notes_dir)
      .unwrap()
      .filter_map(|entry| {
          entry.ok().map(|e| {
              return e.path().file_name().unwrap().to_str().unwrap().to_string();
          })
      })
      .collect()
}

pub fn load_random_note() -> Result<Subtext, NoteError> {
  let notes = list_notes();
  let random_index = rand::random::<usize>() % notes.len();
  let random_note = notes.get(random_index).unwrap();

  return load_note(random_note.to_string());
}