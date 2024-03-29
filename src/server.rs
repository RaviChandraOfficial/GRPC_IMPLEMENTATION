// GRPC= G Remote Proceeding Calls

use tonic::{transport::Server, Request, Response, Status};
use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloResponse, HelloRequest};
// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self,request: Request<HelloRequest>,) -> Result<Response<HelloResponse>, Status> {
        println!("Received request from: {:?}", request);
        let x = request.into_inner();
        let response = greeter::HelloResponse {
            message: format!("Hello: {}, Your gender: {},age: {},company: {},department: {},id: {},location: {}!" ,x.name,x.gender,x.age,x.company,x.department,x.id,x.location).into(),
        };
        Ok(Response::new(response))
    }
}


// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Starting gRPC Server...");
    Server::builder()       //starts building a grpc server instance
        .add_service(GreeterServer::new(greeter))           //adds service to the server
        .serve(addr)        // sepcifies the address to bind the server to.
        .await?;
    Ok(())
}
