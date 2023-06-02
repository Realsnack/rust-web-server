pub fn handle_route(request_path: &str) -> String {
    println!("Request path: {}", request_path);

    if request_path.ends_with("/") {
        println!("Index route");
        return handle_index_route();
    }

    let response = format!("HTTP/1.1 {}\r\n\r\n", 200);
    response
}

fn handle_index_route() -> String {
    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><head><title>Document</title></head><body><p>Hello Workd!</p></body></html>", 200);
    println!("Index route response: {}", response);
    response
}
