use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6920").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.

            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    println!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            // convert bytes to str
            let s = match std::str::from_utf8(&buf[0..n]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("read {} bytes: {}", n, s);

            // Split string by newline
            let mut lines = s.split("\n");

            let first_line = lines.next().unwrap();

            // Match the first word of the first line
            let status_code = match first_line.split(" ").next().unwrap() {
                "GET" => {
                    // Get the second word of the first line
                    let path = first_line.split(" ").nth(1).unwrap();
                    println!("GET request path: {}", path);
                    "200 OK"
                }
                "POST" => {
                    // Get the second word of the first line
                    let path = first_line.split(" ").nth(1).unwrap();
                    println!("POST Request path: {}", path);
                    "200 OK"
                }
                _ => {
                    println!("Invalid request");
                    "500 Internal Server Error"
                }
            };

            // Create response
            let response = format!("HTTP/1.1 {}\r\n\r\n", status_code);

            // Write response to buffer
            buf[..response.len()].copy_from_slice(response.as_bytes());

            if let Err(e) = socket.write_all(&buf[..response.len()]).await {
                println!("failed to write to socket; err = {:?}", e);
                return;
            }
        });
    }
}
