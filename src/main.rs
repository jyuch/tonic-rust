use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

use crate::hello_world::hello_service_server::{HelloService, HelloServiceServer};
use crate::hello_world::{HelloRequest, HelloResponse};

mod hello_world {
    tonic::include_proto!("hello_world");
}

pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let res = HelloResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let hello_service = MyHelloService {};

    tonic::transport::Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .add_service(
            Builder::configure()
                .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                    "descriptor"
                ))
                .build()?,
        )
        .serve(addr)
        .await?;

    Ok(())
}
