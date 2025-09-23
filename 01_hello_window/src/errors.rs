pub enum Error {
    InitError(glfw::InitError),
    CreateWindowError,
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
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<glfw::InitError> for Error {
    fn from(description: glfw::InitError) -> Self {
        Error::InitError(description)
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