pub type ReflectedClientResult<T> = Result<T, ReflectedClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ReflectedClientError {
    #[error(transparent)]
    ProtoBufError(#[from] protobuf::Error),
    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
    #[error(transparent)]
    ReflectionError(#[from] crate::reflection::ReflectionError),
}
