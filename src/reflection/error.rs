pub type ReflectionResult<T> = Result<T, ReflectionError>;

#[derive(Debug, thiserror::Error)]
pub enum ReflectionError {
    #[error(transparent)]
    ClientError(#[from] crate::reflection::client::ClientError),
    #[error(transparent)]
    TonicReflectionError(#[from] tonic_reflection::server::Error),
}
