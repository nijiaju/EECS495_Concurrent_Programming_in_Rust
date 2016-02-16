use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufRead, ErrorKind};
//use std::env;

fn run_server(ip_addr: &str, port: &str) {
    let socket_addr = format!("{}:{}", ip_addr, port);
    let sd: &str = &socket_addr;
    let listener = TcpListener::bind(sd).unwrap();
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

    // get command line if possible
    if let Err(e) = read_buf.read_line(&mut command_line) {
        println!("{}", e);
        return;
    }

    // skip the header and body lines
    let mut discard_data = String::new();
    while let Ok(size) = read_buf.read_line(&mut discard_data) {
        if size == 0 {
            return;
        }
        if discard_data == "\r\n" {
            break;
        }
    }

    // get the command fields
    let fields: Vec<&str> = command_line.split(' ').collect();

    // do not cheack protocol version for forward compatibility

    // prepare response data
    let response = make_response(fields);
    stream = read_buf.into_inner();
    if let Err(e) = stream.write(response.as_bytes()) {
        println!("{}", e);
    }
    return;
}

fn make_response(field: Vec<&str>) -> String {

    // check command
    if field[0] != "GET" {
        return make_response_header("400", "0", false).unwrap();
    }

    // open the file if possible
    //let p = env::current_dir().unwrap();
    //println!("current dir: {}", p.display());
    //println!("file path: {}", field[1]);

    let mut path = field[1];
    let is_html: bool;

    // is html request?
    if  path.ends_with("html") {
        is_html = true;
    } else {
        is_html = false;
    }

    // trim the path
    if path.starts_with("/") {
        path = path.trim_left_matches("/");
    }

    let mut f = match File::open(path) {
        Ok(file)    => file,
        Err(e)      => {
            if e.kind() == ErrorKind::PermissionDenied {
                return make_response_header("403", "0", is_html).unwrap();
            } else {
                return make_response_header("404", "0", is_html).unwrap();
            }
        },
    };

    let mut buf = String::new();
    let size = f.read_to_string(&mut buf).unwrap();
    let mut response = make_response_header("200", &size.to_string(), is_html).unwrap() + "\n";
    response.push_str(&buf);
    return response;
}

fn make_response_header(status_code: &str, length: &str, is_html: bool) 
-> Option<String> {
    let status: &str;

    match status_code {
        "200"   => status = " OK",
        "400"   => status = " Bad Request",
        "403"   => status = " Forbidden",
        "404"   => status = " Not Found",
        _       => return None,
    }
    
    let server = "Server: jns756-web-server/0.0.2";
    let content_type: &str;
    if is_html {
        content_type = "Content-type: text/html";
    } else {
        content_type = "Content-type: text/plain";
    }
    let content_length = format!("Content-Length: {}", length);
    
    // write to log file, only record the valid command
    let mut log = match OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open("server_log") {
        Ok(file)    => file, 
        Err(e)          => {
            println!("write to log file failed: {}", e);
            return None;
        },
    }; 

    // System Time is unstable, ignore
    if let Err(e) = log.write(format!("{}\t{}\n", status, length).as_bytes()) {
        println!("{}", e);
    }
    return Some(format!("HTTP/1.0 {} {}\n{}\n{}\n{}\r\n", 
                        status_code, status, server, content_type, content_length));
}

fn main() {
    run_server("127.0.0.1", "8080");
}
