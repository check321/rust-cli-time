mod client;

use std::env;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8088".to_string());

    println!("Server is listening on: {}", addr);
    let listener = TcpListener::bind(&addr).await?;

    loop {
        // waiting accept.
        // Pattern Matching
        // let (mut sockt, _) = listener.accept().await?;
        let (stream,_) = listener.accept().await?;
        // wrap to frame.
        let mut framed_stream = Framed::new(stream,LengthDelimitedCodec::new());

        // new task from connection.
        tokio::spawn(async move {
            while let Some(msg) = framed_stream.next().await{
                match msg{
                    Ok(msg) => {
                        // parse command.
                        let directive  =  String::from_utf8(msg.to_vec())
                            .expect("occurs an error when converting command to string directive.");
                        println!("{directive}");
                        let output = process(&directive).await;

                        // response to client.
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("{e:?}")
                    }
                }

            }
            // allocate buffer.
        /*    let mut buf = [0; 1024];
            let mut offset = 0;

            loop {

                let lines = sockt.read(&mut buf[offset..])
                    .await
                    .expect("occurs an error when read from socket.");

                // client EOF.
                if lines == 0 {
                    return;
                }

                println!("offset: {offset}, line: {lines}");
                let end = offset + lines;

                // transfer command to string.
                // Pattern Matching
                if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
                    println!("{directive}");
                    // process command.
                    let output = process(directive).await;
                    println!("{output}");

                    // response to client
                    sockt.write_all(&output.as_bytes())
                        .await
                        .expect("occurs an error when response to client");
                }else {
                    offset = end;
                }
            };*/
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        let output = Command::new("date").output().await.unwrap();
        String::from_utf8(output.stdout).unwrap()
    } else {
        "invalid command".to_owned()
    }
}
