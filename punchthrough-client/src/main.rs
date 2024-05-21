use std::net;
//use std::env;
use std::net::SocketAddr;
//use std::net::ToSocketAddrs;
//use std::process::exit;
//use std::thread::Thread;
use std::time::Duration;
//use std::time::Duration;


fn main() {
    let local_address: Vec<SocketAddr> = vec![
        SocketAddr::from( ([0, 0, 0, 0], 8000) ), 
        SocketAddr::from( ([0, 0, 0, 0], 8001) )
    ];
    let target_address: SocketAddr = SocketAddr::from( ([0, 1, 2, 3], 8080) );

    println!("\nTrying to connect to: \"{}\"", target_address);

    if let Ok(socket) = net::UdpSocket::bind(&local_address[..]) {
        println!("Using Socket: {:?}", socket);

        let mut ip_buffer: Vec<u8> = vec![0; 64];

        let message = b"CONNECTION_REQUEST";

        let mut peer_buffer: Vec<u8> = vec![0; 64];

        //let _ = socket.connect(target_address);
        let first_message = socket.send_to(message, target_address);

        if let Ok(_) = first_message {

            println!("sent message to match-maker");
            loop {
                if let Ok(_) = socket.recv_from(&mut ip_buffer) {

                    println!("recieved message from match-maker");

                    let message = String::from_utf8(ip_buffer).expect("Should be a valid string");

                    println!("Recieved: \"{}\"", message);

                    let message = message.trim();
                    let mut temp = message.split(':');
                    let (ip, port) = (temp.next().expect("unable to read IP"), temp.next().expect("Unable to read PORT"));
                    let port = port.trim();
                    let port = port.trim_end_matches('\0');
                    println!("IP: {ip:?}\nPORT: {port:?}");

                    let vec = ip.split('.').map(|val| val.parse::<u8>().expect("Should be valid")).collect::<Vec<u8>>();
                    let mut arr: [u8; 4] = [0; 4];

                    arr[0] = vec[0];
                    arr[1] = vec[1];
                    arr[2] = vec[2];
                    arr[3] = vec[3];


                    let peer_ip = SocketAddr::from( (arr, port.parse::<u16>().expect("Unable to parse port")) );

                    //let peer_ip = message.to_socket_addrs().expect("should be a valid address").next().expect("Should have a valid address");

                    println!("Peer_ip recieved: {}", peer_ip);

                    if let Ok(ttl) = socket.ttl() {
                        let _ = socket.set_ttl(2);
                        let _ = socket.send_to(b"open_connection", message);
                        let _ = socket.set_ttl(ttl);
                    } else {
                        let _ = socket.send_to(b"open_connection", message);
                    }

                    println!("Low time to live packet sent, waiting 1 seconds");
                    std::thread::sleep(Duration::from_secs(1));
                    println!("Commence screaming into the void every second.");

                    loop {
                        println!("LOOPING!");
                        if let Ok(_) = socket.send_to(b"scream into the void", peer_ip) {
                            if let Ok(_) = socket.recv_from(&mut peer_buffer) {
                                let message = String::from_utf8(peer_buffer.to_owned()).expect("Should be a valid string");

                                println!("Recieved: {message}");
                            }
                        }
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        } else {
            println!("{first_message:?}");
        }
    } else { 
        println!("Failed to bind to port 8000")
    }

}