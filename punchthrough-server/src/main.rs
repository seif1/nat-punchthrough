use std::net::{self, SocketAddr};

fn main() {

    println!("\nAttempting to start server\n");

    let socket = net::UdpSocket::bind("0.0.0.0:8080").expect("Unable to bind");
    println!("{socket:?}");
    let mut client1: Option<SocketAddr> = None;
    let mut client2: Option<SocketAddr> = None;

    let mut msg_buff: Vec<u8> = vec![0; 32];
    loop {
        if let Ok(message) = socket.recv_from(&mut msg_buff) {
            let ip = message.1;

            if client1.is_none() {
                client1 = Some(ip);
            } else if client2.is_none() {
                if let Some(client1_addr) = client1 {
                    if ip != client1_addr {
                        client2 = Some(ip);
                    }
                }
            }

            if client1.is_some() && client2.is_some() {
                println!("Sending: {}, to {}", client1.unwrap(), client2.unwrap());
                println!("Sending: {}, to {}", client2.unwrap(), client1.unwrap());

                let _ = socket.send_to(client2.unwrap().to_string().as_bytes(), client1.unwrap()).unwrap();
                let _ = socket.send_to(client1.unwrap().to_string().as_bytes(), client2.unwrap()).unwrap();
                break;
            } else {
                println!("Message from peer: \"{}\" without anything to send.", message.1);
            }
        }
    }

}
