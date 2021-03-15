pub struct RadError(String);

impl std::fmt::Debug for RadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn error<T>(msg: T) -> RadError
where
    T: ToString,
{
    RadError(msg.to_string())
}

impl From<std::io::Error> for RadError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, RadError>;
