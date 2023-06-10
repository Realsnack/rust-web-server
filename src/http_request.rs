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

pub fn parse_request(request: &str) -> Result<HttpRequest, String> {
    let request_lines = request.split("\n");

    // Should look like this:
    // GET /index.html?name=foo HTTP/1.1
    let first_line = request_lines.clone().next().unwrap();

    let method = first_line.split(" ").nth(0).unwrap();
    let path = first_line.split(" ").nth(1).unwrap().split("?").nth(0).unwrap();
    let query_string = first_line.split("?").nth(1).map(|s| s.split(" ").nth(0).unwrap().to_string());
    let protocol = first_line.split(" ").nth(2).unwrap();

    let mut headers: HashMap<String, String> = HashMap::new();
    for line in request_lines.clone().skip(1) {
        if line == "\r" {
            break;
        }

        let mut header = line.split(": ");
        let key = header.next().unwrap();
        let value = header.next().unwrap();

        headers.insert(key.to_string(), value.trim().to_string());
    }
    
    let header_count = headers.len();
    let mut body = String::new();

    for line in request_lines.clone().skip(2+header_count) {
        if line == "\r" {
            break;
        }

        body.push_str(line);
    }

    return Ok(HttpRequest {
        method: method.to_string(),
        path: path.to_string(),
        query_string: query_string.map(|s| s.to_string()),
        protocol: protocol.to_string(),
        headers,
        body: match body.len() {
            0 => None,
            _ => Some(body),
        }
    });
}
