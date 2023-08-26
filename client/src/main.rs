use std::{
    io::{self, Write},
    net::{SocketAddr, TcpStream},
    thread,
};

const LOCALHOST: [u8; 4] = [127, 0, 0, 1];

fn main() {
    print!("Enter port number: ");
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    input_text = input_text.trim().to_string();

    let port: u16;
    match input_text.parse::<u16>() {
        Ok(res) => port = res,
        Err(_) => {
            println!("invalid port");
            std::process::exit(1);
        }
    }

    let addr = SocketAddr::from((LOCALHOST, port));
    let stream = TcpStream::connect(addr).unwrap();

    let write_stream = stream.try_clone().unwrap();
    let write_thread = thread::spawn(move || write_to(write_stream));
    // let read_thread = thread::spawn(move || read_from(stream));

    write_thread.join().unwrap();
}

fn write_to(mut socket: TcpStream) {
    loop {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        socket.write(input_text.as_bytes()).unwrap();
        socket.flush().unwrap();
        println!("debug");
    }
}

// fn read_from(mut socket: TcpStream) {}
