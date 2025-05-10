pub type AppResult<T, E = AppError> = Result<T, E>;

#[derive(Debug)]
pub enum AppError {
    /// Error related to the config file
    Config(String),
    /// Runtime error for spawn out commands to the shell
    Command(String),
    IO(std::io::Error),
    /// Catch all runtime error
    Runtime(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(s) => write!(f, "Config error: {s}"),
            AppError::Command(s) => write!(f, "Command error: {s}"),
            AppError::IO(s) => write!(f, "IO Error: {s}"),
            AppError::Runtime(s) => write!(f, "Runtime Error: {s}"),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IO(value)
    }
}

impl From<global_hotkey::Error> for AppError {
    fn from(value: global_hotkey::Error) -> Self {
        AppError::Runtime(value.to_string())
    }
}

impl From<winit::error::EventLoopError> for AppError {
    fn from(value: winit::error::EventLoopError) -> Self {
        AppError::Runtime(value.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(value: toml::de::Error) -> Self {
        AppError::Runtime(value.to_string())
    }
}
