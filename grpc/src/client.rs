use std::thread::sleep;
use std::time::Duration;

use tokio_stream::{iter, Stream};
use tokio_stream::StreamExt;
use tonic::Status;

use pb::news::news_service_client::NewsServiceClient;
use pb::news::TestRequest;

use crate::pb::printer::printer_service_client::PrinterServiceClient;
use crate::pb::printer::Request;
use crate::pb::TestReply;

mod pb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PrinterServiceClient::connect("grpc://[::1]:9999").await?;

    let in_stream = tonic::Request::new(iter(vec![
        Request {
            code: String::from("20211999"),
            message_type: 1,
        },
        Request {
            code: String::from("20211999"),
            message_type: 2,
        },
        Request {
            code: String::from("20211999"),
            message_type: 2,
        },
    ]));

    let stream = client.channel(in_stream).await?;
    let mut stream = stream.into_inner();

    // loop {
    //     match stream.message().await {
    //         Ok(resp) => {
    //             match resp {
    //                 Some(msg) => println!("{}", msg.desc),
    //                 None => {  }
    //             }
    //         }
    //         Err(e) => println!("{:?}", e)
    //     }
    //     // if let Ok(resp) = stream.message().await {
    //     //     if let Some(msg) = resp {
    //     //         println!("{}", msg.desc);
    //     //     }
    //     // } else {
    //     //     println!("连接关闭");
    //     // }
    // }
    while let Some(resp) = stream.message().await? {
        println!("{}", resp.desc);
    }
    // let mut client = NewsServiceClient::connect("grpc://[::1]:9999").await?;

    // let request = tonic::Request::new(TestRequest {
    //     id: "1".to_string(),
    //     name: "Tonic111111".into(),
    // });
    //
    // let response = client.test_method(request).await?;
    // println!("RESPONSE={:?}", response.get_ref().res);
    //
    // let request = tonic::Request::new(TestRequest {
    //     id: "3".to_string(),
    //     name: "Tonic333".into(),
    // });
    // let response = client.test_method(request).await?;
    // println!("RESPONSE={:?}", response.get_ref().res);
    //
    //
    // let request = tonic::Request::new(TestRequest {
    //     id: "2".to_string(),
    //     name: "Tonic222".into(),
    // });
    // // let response = client.test_method(request).await?;
    // // println!("RESPONSE={:?}", response.get_ref().res);
    //
    // let stream = client.test_stream_method(request).await?;
    // let mut stream = stream.into_inner();
    //
    // while let Some(msg) = stream.next().await {
    //     println!("{}",msg.unwrap().res)
    // }

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