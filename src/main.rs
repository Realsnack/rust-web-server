use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub mod http_method;
mod http_request;
pub mod route_handler;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6920").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            let size: usize = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    println!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let response = handle_request(buf, size).await;

            // Write response to buffer
            buf[..response.len()].copy_from_slice(response.as_bytes());

            if let Err(e) = socket.write_all(&buf[..response.len()]).await {
                println!("failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}

async fn handle_request(request_buffer: [u8; 1024], request_size: usize) -> String {
    // convert bytes to str
    let string_request = match std::str::from_utf8(&request_buffer[0..request_size]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let request = match http_request::parse_request(string_request).await {
        Ok(request) => request,
        Err(e) => return format!("HTTP/1.1 {} Bad Request\r\n\r\n{}", 400, e),
    };

    route_handler::handle_route(request).await
}
