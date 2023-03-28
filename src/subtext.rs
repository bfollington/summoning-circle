use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct Subtext {
    pub name: String,
    pub headers: Vec<(String, String)>,
    pub content: String,
}

impl Subtext {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
        let file = File::open(file_path.as_ref())?;
        let name = file_path.as_ref().file_name().unwrap().to_str().unwrap().to_string();
        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(Result::unwrap);

        let headers: Vec<(String, String)> = lines
            .by_ref()
            .take_while(|line| !line.trim().is_empty())
            .filter(|line| line.contains(':'))
            .map(|line| {
                let mut parts = line.splitn(2, ':');
                let key = parts.next().unwrap().trim().to_string();
                let value = parts.next().unwrap_or("").trim().to_string();
                (key, value)
            })
            .collect();

        let content = lines.collect::<Vec<String>>().join("\n");

        Ok(Subtext { name, headers, content })
    }
}
