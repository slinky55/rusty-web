pub mod handlers;

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub enum Response {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
    NotImplemented
}

pub static ROOT: &str = "public";

pub fn handle_request(mut s: TcpStream) {
    let reader = BufReader::new(&mut s);

    let lines: Vec<String> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let first = lines.first().unwrap_or(&String::from(""));
    if first.is_empty() {
        handle_error(&mut s, Response::BadRequest);
        return;
    }

    let tokens: Vec<&str> = first.split_whitespace().collect();

    let method = tokens[0];
    let protocol = tokens[2];

    if protocol != "HTTP/1.1" {
        handle_error(&mut s, Response::BadRequest);
        return;
    }

    match method {
        "GET" => handle_get(lines, &mut s),
        // "POST" => handle_post(lines, &mut s),
        // "PUT" => handle_put(lines, &mut s),
        // "DELETE" => handle_delete(lines, &mut s),
        _ => handle_error(&mut s, Response::NotImplemented)
    }
}

fn handle_get(lines: Vec<String>, s: &mut TcpStream) {
    let path =
        lines[0].split_whitespace().collect::<Vec<&str>>()[1];

    let contents: String;

    match path {
        "/" => {
            contents = read_file("");
        }
        _ => {
            contents = read_file(path);
        }
    }

    if contents.is_empty() {
        handle_error(s, Response::NotFound);
        return;
    }

    let response = format!(
        "HTTP/1.0 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: text/html\r\n\
        \r\n\
        {}",
        contents.len(),
        contents
    );

    s.write(response.as_bytes()).unwrap();
    s.flush().unwrap();
}

// fn handle_post(lines: Vec<String>, s: &mut TcpStream) {
//     // ...
// }
//
// fn handle_put(lines: Vec<String>, s: &mut TcpStream) {
//     // ...
// }
//
// fn handle_delete(lines: Vec<String>, s: &mut TcpStream) {
//     // ...
// }

fn handle_error(s: &mut TcpStream, t: Response) {
    let initial = match t {
        Response::Ok => "HTTP/1.0 200 OK\r\n",
        Response::NotFound => "HTTP/1.0 404 Not Found\r\n",
        Response::BadRequest => "HTTP/1.0 400 Bad Request\r\n",
        Response::InternalServerError => "HTTP/1.0 500 Internal Server Error\r\n",
        Response::NotImplemented => "HTTP/1.0 501 Not Implemented\r\n"
    };

    let response = format!(
        "{}\
        Content-Length: 0\r\n\
        Content-Type: text/html\r\n\
        \r\n",
        initial
    );

    s.write(response.as_bytes()).unwrap_or(0);
    s.flush().unwrap_or(());
}

fn read_file(path: &str) -> String {
    let complete = String::from(ROOT) + path + "/index.html";

    fs::read_to_string(complete)
        .unwrap_or(String::from(""))
}