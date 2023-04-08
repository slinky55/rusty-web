use std::net::TcpListener;
use std::thread;

use rusty_web::handle_request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for s in listener.incoming() {
        let s = s.unwrap();

        thread::spawn(move || {
            handle_request(s);
        });
    }
}