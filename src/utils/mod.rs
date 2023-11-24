use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Generic error")]
    GenericError,
}
