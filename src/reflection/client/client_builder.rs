use crate::{Client, ReflectedClientResult};
use std::net::ToSocketAddrs;

#[derive(Clone)]

pub struct ClientBuilder {
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
        Self { span, address }
    }

    pub fn build(self) -> ReflectedClientResult<Client> {
        let ClientBuilder {
            address,
            span: builder_span,
        } = self;
        tracing::debug!("ClientBuilder building Client");

        let client_span = tracing::debug_span!("Client");
        client_span.follows_from(builder_span);
        let _ = client_span.enter();
        let client = Client {
            span: client_span,
            address,
        };
        tracing::debug!("ClientBuilder built new : {:?}", &client);
        Ok(client)
    }

    pub fn with_address(mut self, address: std::net::SocketAddr) -> ReflectedClientResult<Self> {
        self.address = address;
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
