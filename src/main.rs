use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, Copy)]
enum Method {
    Post,
    Get,
}

#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
}

fn main() -> Result<(), Box<impl Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in listener.incoming() {
        let stream = stream;
        match stream {
            Ok(connection) => handle_connection(connection)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<impl Error>> {
    let mut buffer = [0; 512];

    stream.read(&mut buffer)?;

    let request = match parse_request(&buffer) {
        Some(v) => v,
        None => return Err("nothing"),
    };

    let response = create_response(request);
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn parse_request(request: &[u8]) -> Option<Request> {
    match request
        .iter()
        .map(|byte| *byte as char)
        .collect::<String>()
        .split("\r\n")
        .next()
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>()[..]
    {
        [method, path, ..] => {
            let method = HashMap::from([("POST", Method::Post), ("GET", Method::Get)])
                [&method.to_uppercase()[..]]
                .to_owned();

            Some(Request {
                method,
                path: String::from(path),
            })
        }
        _ => None,
    }
}

fn create_response(request: Request) -> String {
    let mut content_type = request.path.split(".").last().unwrap_or("html");
    let content = fs::read_to_string(format!("./public/{}", request.path)).unwrap_or_else(|_| {
        content_type = "html";
        String::from("<h1>Not Found</h1>")
    });

    [
        "HTTP/1.1 200 OK",
        "Server: My Rust Code",
        &format!("Content-Length: {}", content.as_bytes().len()),
        &format!("Content-Type: text/{}", content_type),
        "Connection: Closed",
        "",
        &content[..],
    ]
    .join("\r\n")
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add,
{
    a + b
}
