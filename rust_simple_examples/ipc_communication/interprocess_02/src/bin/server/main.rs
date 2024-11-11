use interprocess::local_socket::{self, traits::ListenerExt, GenericNamespaced, ToNsName};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let socket_name = "rust_socket";
    let socket_nsname = socket_name.to_ns_name::<GenericNamespaced>().unwrap();

    // Configure listener
    let listener_options = local_socket::ListenerOptions::new().name(socket_nsname);

    // Create the listener
    let listener = listener_options.create_sync().unwrap();

    println!("Running server at {}", socket_name);

    let mut buffer = String::with_capacity(128);

    for conn in listener.incoming() {
        let mut conn = BufReader::new(conn.expect("Incomming connection failed"));
        println!("Incoming connection!");

        while conn.read_line(&mut buffer).unwrap() > 0 {
            if buffer.trim() == "END" {
                println!("Client disconnected");
                buffer.clear();
                break;
            }

            println!("Client message: {}", buffer);

            buffer.clear();
        }

        conn.get_mut().write_all(b"Hello from server!").unwrap();
    }

    println!("Server exited");
}
