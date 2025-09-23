pub enum Error {
    InitError(glfw::InitError),
    CreateWindowError,
    ReadFileError(std::io::Error),
    CompileShaderError(String),
    CompileProgramError(String),
    ImageError(image::ImageError),
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitError(description) => {
                write!(f, "Failed to initialize glfw\n{}", description)
            }
            Error::CreateWindowError => {
                write!(f, "Failed to create GLFW window")
            }
            Error::ReadFileError(description) => {
                write!(f, "Failed to read file\n{}", description)
            }
            Error::CompileShaderError(description) => {
                write!(f, "Failed to compile shader\n{}", description)
            }
            Error::CompileProgramError(description) => {
                write!(f, "Failed to compile program\n{}", description)
            }
            Error::ImageError(description) => {
                write!(f, "Failed to open image\n{}", description)
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
    fn from(description: glfw::InitError) -> Self {
        Error::InitError(description)
    }
}

impl From<std::io::Error> for Error {
    fn from(description: std::io::Error) -> Self {
        Error::ReadFileError(description)
    }
}

impl From<image::ImageError> for Error {
    fn from(description: image::ImageError) -> Self {
        Error::ImageError(description)
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
                spdlog::error!("{:?}", err);
                std::process::ExitCode::FAILURE
            },
        }
    }
}