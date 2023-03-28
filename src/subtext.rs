use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct Subtext {
    pub headers: Vec<(String, String)>,
    pub content: String,
}

impl Subtext {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut headers = Vec::new();
        let mut content = String::new();
        let mut in_headers = true;

        while reader.read_line(&mut line)? > 0 {
            if in_headers {
                if line.trim().is_empty() {
                    in_headers = false;
                } else {
                    let mut parts = line.splitn(2, ':');
                    let key = parts.next().unwrap().trim().to_string();
                    let value = parts.next().unwrap_or("").trim().to_string();
                    headers.push((key, value));
                }
            } else {
                content.push_str(&line);
            }
            line.clear();
        }

        Ok(Subtext { headers, content })
    }
}
