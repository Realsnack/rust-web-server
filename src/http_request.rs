use std::{collections::HashMap};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query_string: Option<String>,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parse_request_from_buffer(buffer: Vec<u8>) -> Result<HttpRequest, String> {
    let request = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
    };

    let request_lines = request.split("\n");

    // Should look like this:
    // GET /index.html?name=foo HTTP/1.1
    let first_line = request_lines.clone().next().unwrap();

    let method = first_line.split(" ").nth(0).unwrap();
    let path = first_line.split(" ").nth(1).unwrap();
    let query_string = path.split("?").nth(1);
    let protocol = first_line.split(" ").nth(2).unwrap();

    let mut headers: HashMap<String, String> = HashMap::new();
    for line in request_lines.clone().skip(1) {
        if line == "\r" {
            break;
        }

        let mut header = line.split(": ");
        let key = header.next().unwrap();
        let value = header.next().unwrap();

        headers.insert(key.to_string(), value.to_string());
    }
    
    let header_count = headers.len();

    let body = request_lines.clone().skip(header_count + 1);
    let body = body.collect::<Vec<&str>>().join("\n");

    return Ok(HttpRequest {
        method: method.to_string(),
        path: path.to_string(),
        query_string: query_string.map(|s| s.to_string()),
        protocol: protocol.to_string(),
        headers: headers,
        body: body.to_string().into(),
    });
}
