use std::net;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;



fn main() {
    let local_address: Vec<SocketAddr> = vec![
        SocketAddr::from( ([0, 0, 0, 0], 8000) ), 
        SocketAddr::from( ([0, 0, 0, 0], 8001) ),
        SocketAddr::from( ([0, 0, 0, 0], 8002) ),
        SocketAddr::from( ([0, 0, 0, 0], 8003) )
    ];

    let (channel_tx, channel_rx) = std::sync::mpsc::channel::<String>();

    // Change to proper ip of server and not just a local ip
    let target_address: SocketAddr = SocketAddr::from( ([127, 0, 0, 1], 8080) );

    println!("\nTrying to connect to: \"{}\"", target_address);

    let mut resulting_connection = target_address.clone();

    std::thread::spawn(move || {
        if let Ok(socket) = net::UdpSocket::bind(&local_address[..]) {  
            let _ = socket.set_read_timeout(Some(Duration::from_secs(1)));
    
            println!("{socket:?}");

            println!("Bound {}", socket.local_addr().unwrap());
    
            let mut msg_buff: Vec<u8> = vec![0; 128];
    
            let _ = socket.connect(target_address).expect("Failed to connect");
            let _ = socket.send(b"START_CONNECTION");
    
            loop {
                if let Ok((_, addr)) = socket.recv_from(&mut msg_buff) {
                    if addr == target_address {
                        let message: Vec<&str> = std::str::from_utf8(&msg_buff).expect("Invalid Input").trim().trim_end_matches('\0').split(" ").collect();
                        // Manage the possible message states
                        match message[0] {
                            "PING" => {
                                let _ = socket.send(b"PONG");
                            },
                            "ID" => {
                                println!("ID: {}", message[1]);
                            },
                            "CONNECT" => {
                                println!("{:?}", message);
                                // Unsure why this is warning, it is used, just not here.
                                resulting_connection = SocketAddr::from_str(message[1]).expect("Invalid address from remote");
                                // Break out of connection loop with remote server in order to communicate with intended recipient
                                break;
                            }
                            
                            // We dont handle unknown messages, because this is intended to be simple
                            _ => {
                                println!("Unknown message: {}", std::str::from_utf8(&msg_buff).expect("Invalid Input"));
                            }
                        }
                    }
                } else {
                    println!("error");
                }
                msg_buff.fill(0);
                
                // Inform remote server of intent to connect to peer.
                if let Ok(given_id) = channel_rx.try_recv() {
                    println!("Connecting now");
                    let _ = socket.send(format!("CONNECT {}", given_id).as_bytes());
                }
            }

            // Set up socket for new connection and connect to peer to allow send and recv functions to only operate with that address
            let _ = socket.set_nonblocking(true);
            socket.connect(resulting_connection).unwrap();

            loop {
                // Really here would go some actual message transfer after first contact is made, but this will just repeatedly send messages out.
                println!("Sending messages into the void and hoping for a response");
                let _ = socket.send(b"MESSAGE_INTO_THE_VOID");
                if let Ok(_) = socket.recv(&mut msg_buff) {
                    let message = std::str::from_utf8(&msg_buff).expect("Invalid Input").trim().trim_end_matches('\0');
                    
                    println!("{}", message);
                    msg_buff.fill(0);
                }

                std::thread::sleep(Duration::from_secs(1));
            }
        } else {
            panic!("Unable to bind port 8000, or 8001");
        }
    });

    // Less than ideal read from console thread just to simplify things.
    std::thread::sleep(Duration::from_secs(1));

    let mut input_buffer: String = Default::default();
    println!("Enter partner ID");
    let _ = std::io::stdin().read_line(&mut input_buffer);
    let _ = channel_tx.send(input_buffer.trim().to_owned());
    println!("Attempting to connect to: {}", input_buffer.trim());

    // We dont handle if the peer is incorrect, because this is intended to be simple
    loop {
        std::thread::sleep(Duration::from_secs(50));
    }
}