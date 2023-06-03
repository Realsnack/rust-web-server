pub fn handle_route(request_path: &str) -> String {
    println!("Request path: {}", request_path);

    if request_path.ends_with("/") {
        return handle_index_route();
    }

    let response = format!("HTTP/1.1 {}\r\n\r\n", 200);
    response
}

fn handle_index_route() -> String {
    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><title>Document</title></head><body><p>Hello World!</p></body></html>", 200);
    response
}
