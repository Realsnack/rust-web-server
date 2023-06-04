use std::str::Split;

pub fn handle_route(request_path: &str, request: Split<&str>) -> String {
    if request_path.ends_with("/") {
        return handle_index_route();
    }

    println!("Request: {:?}", request);

    let response = format!("HTTP/1.1 {}\r\n\r\n", 200);
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
