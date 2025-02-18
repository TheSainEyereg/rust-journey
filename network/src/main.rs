use std::{
    io::Read,
    net::{TcpListener, UdpSocket},
    thread,
};

fn main() {
    let udp_handle = thread::spawn(|| {
        let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
        println!("Started UDP server!");

        let mut buf = [0; 16];
        let (_, src) = socket.recv_from(&mut buf).unwrap();

        println!(
            "UDP Received {} from {}",
            String::from_utf8_lossy(&buf),
            src
        );
    });

    let tcp_handle = thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:34254").unwrap();
        println!("Started TCP server!");

        let mut buf = [0; 16];
        let (mut stream, src) = listener.accept().unwrap();
        stream.read(&mut buf).unwrap();

        println!(
            "TCP Received {} from {}",
            String::from_utf8_lossy(&buf),
            src
        )
    });

    udp_handle.join().unwrap();
    tcp_handle.join().unwrap();
}
