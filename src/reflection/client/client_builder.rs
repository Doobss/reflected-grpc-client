use protobuf::descriptor;

use super::ReflectionClient;
use crate::{Client, ReflectedClientResult};
use std::net::ToSocketAddrs;

#[derive(Clone)]

pub struct ClientBuilder {
    secure: bool,
    address: std::net::SocketAddr,
    pub(crate) span: tracing::Span,
}

impl ClientBuilder {
    pub fn new() -> Self {
        let span = tracing::debug_span!("ClientBuilder");
        let _ = span.enter();
        tracing::debug!("Created new ClientBuilder");
        let address = "[::]:50051"
            .to_socket_addrs()
            .expect("Error parsing defautl client address.")
            .next()
            .expect("Missing default socket address.");
        Self {
            span,
            address,
            secure: false,
        }
    }

    pub async fn build(self) -> ReflectedClientResult<Client> {
        let ClientBuilder {
            address,
            span: builder_span,
            secure,
        } = self;
        tracing::debug!("ClientBuilder building Client @ address: {}", &address);

        let client_span = tracing::debug_span!("Client");
        client_span.follows_from(builder_span);
        let _ = client_span.enter();

        let mut reflection_client = ReflectionClient::from_address(address, secure).await?;
        let services = reflection_client.get_services().await?;
        tracing::debug!("reflected services: {:?}", &services);

        let client = Client {
            span: client_span,
            address,
            reflection_client,
        };
        tracing::debug!("ClientBuilder built new : {:?}", &client);
        Ok(client)
    }

    pub fn with_address(mut self, address: std::net::SocketAddr) -> ReflectedClientResult<Self> {
        self.address = address;
        Ok(self)
    }

    pub fn is_secure(mut self, secure: bool) -> ReflectedClientResult<Self> {
        self.secure = secure;
        Ok(self)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("address", &self.address)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new_builder() -> ReflectedClientResult<()> {
        let _ = ClientBuilder::new();
        Ok(())
    }

    #[tokio::test]
    async fn build_default_client() -> ReflectedClientResult<()> {
        let _ = ClientBuilder::new().build().await?;
        Ok(())
    }

    #[tokio::test]
    async fn set_builder_address_and_build_client() -> ReflectedClientResult<()> {
        let builder = ClientBuilder::new().with_address("[::]:50052".parse()?)?;
        let _ = builder.build().await?;
        Ok(())
    }
}
