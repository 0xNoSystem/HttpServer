use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use framework::http::*;
use framework::error::Error;

type Result<T> = std::result::Result<T, Error>;



async fn handle_client_close(mut stream: TcpStream) -> Result<()> {
    let mut buff = [0u8; 1024];
    let n = stream.read(&mut buff).await?;
    let req = String::from_utf8_lossy(&buff[..n]);
    println!("{}", req);
    let request = HttpRequest::parse(req.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    println!("_____REQUEST_____
    {:?}", request);

    let body = "<h1> Hello this is Spongy Bob, assalamu aalaykoun </h1>";
    let res = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(res.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

async fn handle_client_alive(mut stream: TcpStream) -> Result<()> {
    let mut buff = [0u8; 1024];

        let n = stream.read(&mut buff).await?;
        let req = String::from_utf8_lossy(&buff[..n]);
        println!("{}", req);
        let request = HttpRequest::parse(req.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    println!("_____REQUEST_____
 {:?}", request);

    let body = "<h1> Hello this is Spongy Bob, assalamu aalaykoun </h1>";
    let res = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\nConnection: keep-alive\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(res.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}






#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on http://localhost:8080");

    while let Ok((stream, _address)) =  listener.accept().await {
            if let Err(e) = tokio::spawn(async move {
                                            handle_client_close(stream).await
                                        }).await.unwrap() {
                    eprintln!("Client error: {}", e);
        }
    }
    Ok(())
}








