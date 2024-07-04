use reflected_grpc_client::{logging, reflection::Client, ReflectedClientResult};

#[tokio::main]
async fn main() -> ReflectedClientResult<()> {
    logging::init();

    let client_builder = Client::builder().with_address("[::]:50052".parse()?)?;
    let _client = client_builder.build()?;

    Ok(())
}
