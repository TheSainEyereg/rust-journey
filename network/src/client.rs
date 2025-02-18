use std::{
    env,
    io::Write,
    net::{TcpStream, UdpSocket},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = &args[1];

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect((address.clone(), 34254)).unwrap();
    socket.send("1234".as_bytes()).unwrap();

    let mut stream = TcpStream::connect((address.clone(), 34254)).unwrap();
    stream.write("5678".as_bytes()).unwrap();
}
