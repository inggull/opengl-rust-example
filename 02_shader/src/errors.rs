pub enum Error {
    InitError(glfw::InitError),
    CreateWindowError,
    ReadFileError(std::io::Error),
    CompileShaderError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitError(reason) => {
                spdlog::error!("Failed to initialize glfw");
                write!(f, "{}", reason)
            }
            Error::CreateWindowError => {
                spdlog::error!("Failed to create GLFW window");
                write!(f, "")
            }
            Error::ReadFileError(reason) => {
                spdlog::error!("Failed to read file");
                write!(f, "{}", reason)
            }
            Error::CompileShaderError(reason) => {
                spdlog::error!("Failed to compile shader");
                write!(f, "{}", reason)
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<glfw::InitError> for Error {
    fn from(reason: glfw::InitError) -> Self {
        Error::InitError(reason)
    }
}

impl From<std::io::Error> for Error {
    fn from(reason: std::io::Error) -> Self {
        Error::ReadFileError(reason)
    }
}

pub struct Result<T, E>(std::result::Result<T, E>);

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(value: std::result::Result<T, E>) -> Self {
        Self(value)
    }
}

impl<T, E> std::process::Termination for Result<T, E> where T: std::process::Termination, E: std::fmt::Debug {
    fn report(self) -> std::process::ExitCode {
        match self.0 {
            Ok(val) => val.report(),
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::ExitCode::FAILURE
            },
        }
    }
}