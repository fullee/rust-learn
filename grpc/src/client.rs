mod pb;

// use crate::pb::greeter_client::GreeterClient;
// // use hello_world::HelloRequest;
// use crate::pb::HelloRequest;

use tonic::Status;
use pb::news::news_service_client::NewsServiceClient;
use pb::news::TestRequest;
use crate::pb::TestReply;

// pub mod hello_world {
//     tonic::include_proto!("helloworld");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NewsServiceClient::connect("grpc://[::1]:9999").await?;

    let request = tonic::Request::new(TestRequest {
        id: "1".to_string(),
        name: "Tonic111111".into(),
    });

    let response = client.test_method(request).await?;
    println!("RESPONSE={:?}", response.get_ref().res);

    let request = tonic::Request::new(TestRequest {
        id: "3".to_string(),
        name: "Tonic333".into(),
    });
    let response = client.test_method(request).await?;
    println!("RESPONSE={:?}", response.get_ref().res);


    let request = tonic::Request::new(TestRequest {
        id: "2".to_string(),
        name: "Tonic222".into(),
    });
    // let response = client.test_method(request).await?;
    // println!("RESPONSE={:?}", response.get_ref().res);

    let stream = client.test_stream_method(request).await?;
    let mut stream = stream.into_inner();

    while let Some(msg) = stream.message().await.unwrap_or_else("出错了") {
        println!("{}",msg.res)
    }

    // loop {
    //     match stream.message().await {
    //         Ok(tr) => {
    //             if let Some(msg) = tr {
    //                 println!("{}",msg.res)
    //             } else {
    //                 println!("空消息");
    //                 break;
    //             }
    //         }
    //         Err(_) => {
    //             println!("报错了");
    //             break;
    //         }
    //     }
    // }


    Ok(())
}