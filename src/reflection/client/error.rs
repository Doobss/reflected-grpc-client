pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {}
