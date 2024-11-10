use interprocess::local_socket::{self, traits::Stream, GenericNamespaced, ToNsName};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let socket_name = "rust_socket";
    let socket_nsname = socket_name.to_ns_name::<GenericNamespaced>().unwrap();

    let mut buffer = String::with_capacity(128);

    let conn = local_socket::Stream::connect(socket_nsname).expect("Failed to connect to server. ");
    let mut conn = BufReader::new(conn);
    conn.get_mut().write_all(b"Hello from client!\n").unwrap();

    conn.read_line(&mut buffer).unwrap();
    println!("Server message: {}", buffer);

    println!("Client exited");
}
