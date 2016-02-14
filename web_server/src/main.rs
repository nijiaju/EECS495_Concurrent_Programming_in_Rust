use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead, ErrorKind};

fn run_server(port: &str, is_local_host: bool) {
    if !is_local_host {
        return;
    }

    let socket_addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(socket_addr).unwrap();
    println!("started listening on {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }
    } 
    drop(listener);
}
    
fn handle_client(mut stream: TcpStream) {
    let mut read_buf = BufReader::new(stream);
    let mut command_line = String::new();

    // get command line, ignore HTTP head
    read_buf.read_line(&mut command_line);
    let fields: Vec<&str> = command_line.split(' ').collect();

    // prepare response data
    let response: String;
    match fields[0] {
        "GET"   => response = make_response(fields[1]),
        _       => response = make_response_header("400", "0").unwrap(),
    }

    stream = read_buf.into_inner();
    stream.write(response.as_bytes());
    return;
}

fn make_response(path: &str) -> String {
    let mut f = match File::open(path) {
        Ok(file)    => file,
        Err(e)      => {
            if e.kind() == ErrorKind::PermissionDenied {
                return make_response_header("403", "0").unwrap();
            } else {
                return make_response_header("404", "0").unwrap();
            }
        },
    };

    let mut buf = String::new();
    let size = f.read_to_string(&mut buf).unwrap();
    let mut response = make_response_header("200", &size.to_string()).unwrap() + "\n";
    
    response.push_str(&buf);
    return response;
}

fn make_response_header(status_code: &str, content_length: &str) -> Option<String> {
    let mut response_header = String::new();

    response_header = response_header + "HTTP/1.0 " + status_code;
    match status_code {
        "200"   => response_header = response_header + " OK\n",
        "400"   => response_header = response_header + " Bad Request\n",
        "403"   => response_header = response_header + " Forbidden\n",
        "404"   => response_header = response_header + " Not Found\n",
        _       => return None,
    }
    response_header = response_header + "Server: jns756-web-server/0.0.1\n";
    response_header = response_header + "Content-type: text/plain";
    response_header = response_header + "Content-Length: " + content_length + "\n";

    return Some(response_header);
}

fn main() {
    run_server("8080", true);
}
