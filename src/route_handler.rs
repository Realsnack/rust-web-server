use std::str::Split;

pub fn handle_route(request_path: &str, request: Split<&str>) -> String {
    // convert request to vector
    let request: Vec<&str> = request.collect();
    println!("Request: {:?}", request[0]);

    if !request[0].starts_with("GET"){
        return format!("HTTP/1.1 {} Method Not Found\r\n\r\n<!DOCTYPE html><html lang=\"en\"><head>    <meta charset=\"UTF-8\">    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">    <title>Method Not Allowed</title></head><body>    <h1>Method Not Allowed</h1>    <p>The method specified in the request is not allowed for the resource identified by the request URI.</p></body></html>", 405);
    }

    if request_path.ends_with("/") {
        return handle_index_route();
    }

    println!("Request: {:?}", request);

    let response = format!("HTTP/1.1 {} Not Found\r\n\r\n", 404);
    response
}

fn handle_index_route() -> String {
    let content = "<!DOCTYPE html><html><head><title>Document</title></head><body><p>Hello World!</p></body></html>";
    let content_length = content.len();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        200, content_length, content
    );
    response
}
