
pub struct Environment {
    pub api_path: String,
    pub api_key: String,
}

impl Environment {
  pub fn from_env() -> Result<Self, std::env::VarError> {
    let api_path = std::env::var("API_PATH")?;
    let api_key = std::env::var("API_KEY")?;

    Ok(Environment { api_path, api_key })
  }
}