#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TelepassError {
    #[error("Auth date missing")]
    AuthDateMissing,
    #[error("Auth date invalid")]
    AuthDateInvalid,
    #[error("Sign missing")]
    SignMissing,
    #[error("Sign invalid")]
    SignInvalid,
    #[error("Unexpected format")]
    UnexpectedFormat,
    #[error("Init data expired")]
    Expired,
}
