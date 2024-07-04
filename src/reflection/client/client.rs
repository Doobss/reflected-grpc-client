use super::ClientBuilder;

#[derive(Clone)]

pub struct Client {
    pub(crate) address: std::net::SocketAddr,
    pub(crate) span: tracing::Span,
}

impl Client {
    pub fn address(&self) -> &std::net::SocketAddr {
        &self.address
    }

    pub fn ip(&self) -> std::net::IpAddr {
        self.address.ip()
    }

    pub fn port(&self) -> u16 {
        self.address.port()
    }

    pub fn span(&self) -> &tracing::Span {
        &self.span
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

impl core::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("address", &self.address)
            .finish()
    }
}
