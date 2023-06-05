[![build and test](https://github.com/Realsnack/rust-web-server/actions/workflows/rust.yaml/badge.svg?branch=main)](https://github.com/Realsnack/rust-web-server/actions/workflows/rust.yaml)

# Simple Web Server in Rust

## Overview

The purpose of this project is to develop a simple web server using the Rust programming language. This will help me to gain understanding and expertise in Rust, a high-performance system programming language. It will also serve as a foundation for potential future projects that might require a custom web server for specific needs.

## Goals

1. The web server should be able to handle HTTP requests (GET, POST).
2. The web server should provide informative responses for different routes.
3. The web server should have basic error handling capabilities (e.g. 404 Not Found errors).
4. The web server should be able to serve static files from a defined directory.

## Scope

The following items are included within the scope of this project:

- A web server developed in Rust.
- An ability to handle basic HTTP methods, specifically GET and POST requests.
- Basic error handling, such as responding with a 404 status code when a resource is not found.
- Ability to serve static files.

Out of scope:

- Database integration, user authentication, and session management.
- Advanced error handling or logging functionalities.
- HTTPS support or other security features.

## Business Requirements

### Requirement 1: HTTP Request Handling

The web server must accept and properly handle HTTP GET and POST requests. 

### Requirement 2: Route Handling

The web server must respond to specific routes with appropriate HTTP responses. As a minimum, the server should handle a root ("/") route and a 404 route.

### Requirement 3: Error Handling

The server must respond with a 404 Not Found status code and a basic error page when a user attempts to access a resource that does not exist.

### Requirement 4: Static File Serving

The server must be able to serve static files from a defined directory. This will allow the server to deliver HTML, CSS, and JavaScript files to the client.

## Future Considerations

Future enhancements to the server could include:

- Integration with a database system for data persistence.
- User authentication and session management functionalities.
- Implementation of HTTPS and other security best practices.
- Addition of a logging system for easier monitoring and debugging.
