pub type ReflectedClientResult<T> = Result<T, ReflectedClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ReflectedClientError {
    #[error(transparent)]
    ProtoBufError(#[from] protobuf::Error),
    #[error(transparent)]
    TonicTransportError(#[from] tonic::transport::Error),
    // #[error(transparent)]
    // TonicReflectionClientError(#[from] tonic_reflection::pb::ErrorResponse),
    #[error(transparent)]
    TonicServiceReflectionError(#[from] tonic_reflection::server::Error),
    #[error(transparent)]
    TonicRequestError(#[from] tonic::Status),
    #[error(transparent)]
    ParseAddressError(#[from] std::net::AddrParseError),
    #[error("Error parsing Uri for tonic transport: {message}")]
    ParseUriError { message: String },
    #[error("Error with reflected client: {message}")]
    ReflectionError { message: String },
}
