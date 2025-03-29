use std::ffi::OsStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub telegram_token: String,
    pub database_path: String,
    pub api_key: ApiKeyConfig,
}

#[derive(Debug, Clone)]
pub struct ApiKeyConfig {
    pub gemini_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Config, std::env::VarError> {
        Ok(Config {
            telegram_token: env_var("TELEGRAM_TOKEN")?,
            database_path: env_var("DATABASE_PATH")?,
            api_key: ApiKeyConfig {
                gemini_key: env_var_nullable("GEMINI_API_KEY")?,
            },
        })
    }
}

fn env_var_nullable(key: impl AsRef<OsStr>) -> Result<Option<String>, std::env::VarError> {
    Ok(match std::env::var(key) {
        Ok(key) => Some(key),
        Err(err) => match err {
            std::env::VarError::NotPresent => None,
            err => return Err(err),
        },
    })
}

fn env_var(key: impl AsRef<OsStr>) -> Result<String, std::env::VarError> {
    std::env::var(key)
}
