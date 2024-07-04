use std::collections::HashMap;
use std::str::FromStr;

use crate::{ReflectedClientError, ReflectedClientResult, Service};
use protobuf::{descriptor, Message};
use tokio_stream::StreamExt;
use tonic::transport::Uri;
use tonic_reflection::pb::{
    server_reflection_client::ServerReflectionClient, server_reflection_request::MessageRequest,
    FileDescriptorResponse, ServerReflectionRequest, ServerReflectionResponse, ServiceResponse,
};

#[derive(Clone, Debug)]
pub(crate) struct ReflectionClient {
    endpoint: tonic::transport::Endpoint,
    inner: tonic_reflection::pb::server_reflection_client::ServerReflectionClient<
        tonic::transport::Channel,
    >,
}

impl ReflectionClient {
    pub async fn from_address(
        address: std::net::SocketAddr,
        secure: bool,
    ) -> ReflectedClientResult<Self> {
        let address = if secure {
            format!("https://{}", address)
        } else {
            format!("http://{}", address)
        };
        let uri = Uri::from_str(&address).map_err(|error| ReflectedClientError::ParseUriError {
            message: error.to_string(),
        })?;
        let builder = tonic::transport::Channel::builder(uri);
        Self::from_endpoint(builder).await
    }

    pub async fn from_endpoint(
        endpoint: tonic::transport::Endpoint,
    ) -> ReflectedClientResult<Self> {
        let connection = endpoint.connect().await?;
        let inner = ServerReflectionClient::new(connection);
        Ok(Self { inner, endpoint })
    }

    pub async fn get_services(&mut self) -> ReflectedClientResult<HashMap<String, Service>> {
        let services = self.get_service_descriptors().await?;
        let descriptors = self.get_descriptors(services).await?;
        let mut services = HashMap::with_capacity(descriptors.keys().len());
        for descriptor in descriptors.into_values() {
            for service_descriptor in descriptor.services() {
                let service_name = service_descriptor.proto().name().to_owned();
                let service = Service::from_descriptor(service_descriptor)?;
                services.insert(service_name, service);
            }
        }
        Ok(services)
    }

    pub(crate) async fn get_service_descriptors(
        &mut self,
    ) -> ReflectedClientResult<Vec<ServiceResponse>> {
        let request_stream = tokio_stream::iter(vec![ServerReflectionRequest {
            host: self
                .endpoint
                .uri()
                .host()
                .expect("Uri has no host.")
                .to_owned(),
            message_request: Some(MessageRequest::ListServices("".to_owned())),
        }]);
        let mut stream = self
            .inner
            .server_reflection_info(request_stream)
            .await?
            .into_inner();

        while let Some(message) = stream.next().await {
            let message = message?;
            let ServerReflectionResponse {
                message_response, ..
            } = message;
            if let Some(response) = message_response {
                use tonic_reflection::pb::server_reflection_response::MessageResponse;
                match response {
                    MessageResponse::ListServicesResponse(response) => {
                        return Ok(response
                            .service
                            .into_iter()
                            .filter(|service| {
                                !service.name.clone().to_lowercase().contains("reflection")
                            })
                            .collect());
                    }
                    _ => {
                        tracing::info!("received incorrect response: {:?}", &response);
                    }
                }
            }
        }
        Err(ReflectedClientError::ReflectionError {
            message: "Never recived list services response.".to_owned(),
        })
    }

    pub(crate) async fn get_descriptors(
        &mut self,
        services: Vec<ServiceResponse>,
    ) -> ReflectedClientResult<HashMap<String, protobuf::reflect::FileDescriptor>> {
        let mut descriptors = HashMap::with_capacity(services.len());
        let requests: Vec<ServerReflectionRequest> = services
            .into_iter()
            .map(|service| ServerReflectionRequest {
                host: self
                    .endpoint
                    .uri()
                    .host()
                    .expect("Uri has no host.")
                    .to_owned(),
                message_request: Some(MessageRequest::FileContainingSymbol(service.name)),
            })
            .collect();
        let request_stream = tokio_stream::iter(requests);
        let mut stream = self
            .inner
            .server_reflection_info(request_stream)
            .await?
            .into_inner();

        while let Some(message) = stream.next().await {
            let message = message?;
            let ServerReflectionResponse {
                original_request,
                message_response,
                ..
            } = message;
            if let Some(original_request) = original_request {
                let ServerReflectionRequest {
                    message_request,
                    host,
                } = original_request;
                let service_name = if let Some(request) = message_request.clone() {
                    match request {
                        MessageRequest::FileContainingSymbol(service_name) => Some(service_name),
                        _ => None,
                    }
                } else {
                    None
                };
                if let Some(service_name) = service_name {
                    if let Some(response) = message_response {
                        use tonic_reflection::pb::server_reflection_response::MessageResponse;
                        match response {
                            MessageResponse::FileDescriptorResponse(response) => {
                                let FileDescriptorResponse {
                                    file_descriptor_proto,
                                } = response;
                                if !file_descriptor_proto.is_empty() {
                                    let descriptor = protobuf::descriptor::FileDescriptorProto::parse_from_bytes(&file_descriptor_proto[0])?;
                                    tracing::debug!(
                                        "Parsed descriptor: {:?} for service: {:?}",
                                        &descriptor.name,
                                        &service_name,
                                    );
                                    let descriptor =
                                        protobuf::reflect::FileDescriptor::new_dynamic(
                                            descriptor,
                                            &[],
                                        )?;

                                    tracing::debug!(
                                        "Created dynamic descriptor: {:?} for service: {:?}",
                                        &descriptor.name(),
                                        &service_name,
                                    );
                                    descriptors.insert(service_name, descriptor);
                                } else {
                                    tracing::warn!("FileDescriptorResponse was empty");
                                }
                            }
                            _ => {
                                tracing::warn!("received incorrect response: {:?}", &response);
                            }
                        }
                    }
                } else {
                    tracing::warn!(
                        "no service name found in request: {:?} @ host: {:?}",
                        message_request,
                        host
                    );
                }
            } else {
                tracing::warn!("original request not given: {:?}", original_request);
            }
        }
        Ok(descriptors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn build_reflection_client_from_address() -> ReflectedClientResult<()> {
        let _ = ReflectionClient::from_address("[::]:50051".parse()?, false).await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_services() -> ReflectedClientResult<()> {
        let mut client = ReflectionClient::from_address("[::]:50051".parse()?, false).await?;
        let services = client.get_service_descriptors().await?;
        assert_ne!(services.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn get_descriptors() -> ReflectedClientResult<()> {
        let mut client = ReflectionClient::from_address("[::]:50051".parse()?, false).await?;
        let services = client.get_service_descriptors().await?;
        let number_of_services = services.len();
        let descriptors = client.get_descriptors(services).await?;
        let number_of_descriptors = descriptors.keys().len();
        assert_eq!(number_of_services, number_of_descriptors);
        Ok(())
    }
}
