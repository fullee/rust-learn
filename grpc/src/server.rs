mod pb;

use tonic::{transport::Server, Request, Response, Status};

use pb::news::{TestRequest,TestReply};
use pb::news_service_server::{NewsServiceServer,NewsService};

// pub mod hello_world {
//     tonic::include_proto!("helloworld");
// }

#[derive(Debug, Default)]
pub struct MyGreeter {}

// #[tonic::async_trait]
// impl NewsService for MyGreeter {
//     async fn test_method(&self, request: Request<TestRequest>) -> Result<Response<TestReply>, Status> {
//         println!("Got a request: {:?}", request);
//
//         let reply = hello_world::HelloReply {
//             message: format!("Hello {}!", request.into_inner().name).into(),
//         };
//
//         Ok(Response::new(reply))
//     }
//
//     type testStreamMethodStream = ();
//
//     async fn test_stream_method(&self, request: Request<TestRequest>) -> Result<Response<Self::testStreamMethodStream>, Status> {
//         todo!()
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;
    // let greeter = MyGreeter::default();
    //
    // Server::builder()
    //     .add_service(NewsServiceServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}