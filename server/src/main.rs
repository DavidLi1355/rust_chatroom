use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    str,
    sync::{Arc, Mutex},
    thread,
};

const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
const BUFFER_SIZE: usize = 20;

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

    let listener = TcpListener::bind(addr).unwrap();
    // listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("connection from {addr:?}");
                clients.lock().unwrap().push(socket.try_clone().unwrap());
                println!("{clients:?}");

                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || listen_for_msg(socket, clients_clone));
            }
            Err(_) => {}
        }
    }

}

fn listen_for_msg(mut socket: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    loop {
        let mut buffer = vec![0; BUFFER_SIZE];
        match socket.read(&mut buffer) {
            Ok(res) => match res {
                0 => break,
                _ => {
                    let msg = str::from_utf8(&buffer).unwrap();
                    println!("recv msg: {msg}");
                    {
                        for mut c in clients.lock().unwrap().iter() {
                            println!("writing msg to {c:?}");
                            c.write(&buffer).unwrap();
                        }
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }
}

