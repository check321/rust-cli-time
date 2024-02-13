use std::env;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8088".to_string());

    // connect to server.
    let stream = TcpStream::connect(&addr).await?;

    // write command to server.
    // stream.write_all(b"gettime").await?;

    // wrap to frame.
    let mut framed_stream = Framed::new(stream,LengthDelimitedCodec::new());
    // send command to server.
    framed_stream.send(Bytes::from("gettime")).await?;

    // read response from server in once.
    if let Some(msg) = framed_stream.next().await{
        match msg {
            Ok(msg) => {
                let timeinfo = String::from_utf8(msg.to_vec())?;
                println!("{}",timeinfo);
            }
            Err(e) => return Err(e.into()),
        }
    }

    /*// wait response from server.
    let mut buf: Vec<u8> = Vec::with_capacity(9216);
    // read buffer.
    let mut resp = [0u8;2048];
    loop{
        // read once.
        let lines = stream.read(&mut resp).await?;
        // merge read bytes to buffer.
        buf.extend_from_slice(&resp[0..lines]);

        // EOF
        if(lines == 0){
            panic!("Unexpected EOF");
        }else if buf.len() >= 48{
            // 2024年 2月13日 星期二 22时08分44秒 CST
            break;
        }else{
            // fill in buf.
            continue;
        }
    }

    let timeinfo = String::from_utf8(buf)?;
    println!("{}",timeinfo);*/

    Ok(())
}