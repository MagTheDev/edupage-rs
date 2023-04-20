
use derive_more::Display;



#[derive(Debug, Display)]
pub enum EdupageError {
    #[display(fmt = "Incorrect Credentials Provided")]
    LoginError,
    #[display(fmt = "A Network Error Occured")]
    RequestError(reqwest::Error),
}

impl From<reqwest::Error> for EdupageError {
    fn from(value: reqwest::Error) -> Self {
        return Self::RequestError(value);
    }
}

pub enum DeserializeError {
    UnknownType(String)
}

