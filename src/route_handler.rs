use crate::{http_method::HttpMethod, http_request::HttpRequest};

pub async fn handle_route(request: HttpRequest) -> String {
    let response = match request.path.as_str() {
        "/" => handle_index_route(request).await,
        _ => format!("HTTP/1.1 {} Not Found\r\nServer: Your-Mom-420x69\r\n", 404),
    };

    response
}

async fn handle_index_route(request: HttpRequest) -> String {
    match request.method {
        HttpMethod::GET => (),
        _ => return format!("HTTP/1.1 {} Method Not Allowed\r\n\r\n<!DOCTYPE html><html lang=\"en\"><head>    <meta charset=\"UTF-8\">    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">    <title>Method Not Allowed</title></head><body>    <h1>Method Not Allowed</h1>    <p>The method specified in the request is not allowed for the resource identified by the request URI.</p></body></html>", 405),
    }

    let content = "<!DOCTYPE html><html><head><title>Document</title></head><body><p>Hello World!</p></body></html>";
    let content_length = content.len();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        200, content_length, content
    );

    response
}
