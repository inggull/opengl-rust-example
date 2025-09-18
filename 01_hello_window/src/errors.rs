pub enum Error {
    InitError(glfw::InitError),
    CreateWindowError,
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitError(reason) => {
                spdlog::error!("Failed to initialize glfw, Reason: {}", reason);
                write!(f, "Failed to initialize glfw, Reason: {}", reason)
            }
            Error::CreateWindowError => {
                spdlog::error!("Failed to create GLFW window");
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
    fn from(reason: glfw::InitError) -> Self {
        Error::InitError(reason)
    }
}
