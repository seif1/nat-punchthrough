use std::io::Read;
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
            //let _ = socket.set_read_timeout(Some(Duration::from_secs(3)));
    
            println!("Bound {}", socket.local_addr().unwrap());
    
            let mut msg_buff: Vec<u8> = vec![0; 128];
    
            let _ = socket.connect(target_address).expect("Failed to connect");
            let _ = socket.send(b"START_CONNECTION");
    
            loop {
                if let Ok((_, addr)) = socket.recv_from(&mut msg_buff) {
                    if addr == target_address {
                        let message: Vec<&str> = std::str::from_utf8(&msg_buff).expect("Invalid Input").trim().trim_end_matches('\0').split(" ").collect();
                        match message[0] {
                            "PING" => {
                                let _ = socket.send(b"PONG");
                            },
                            "ID" => {
                                println!("ID: {}", message[1]);
                            },
                            "CONNECT" => {
                                println!("{:?}", message);
                                resulting_connection = SocketAddr::from_str(message[1]).expect("Invalid address from remote");
                                break;
                            }
                            
                            _ => {
                                println!("Unknown message: {}", std::str::from_utf8(&msg_buff).expect("Invalid Input"));
                            }
                        }
                    }
                } else {
                    println!("error");
                }
                msg_buff.fill(0);
                
                if let Ok(given_id) = channel_rx.try_recv() {
                    println!("Connecting now");
                    let _ = socket.send(format!("CONNECT {}", given_id).as_bytes());
                }
            }

            let _ = socket.set_nonblocking(true);
            socket.connect(resulting_connection).unwrap();

            loop {
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

    std::thread::sleep(Duration::from_secs(1));

    let mut input_buffer: String = Default::default();
    println!("Enter partner ID");
    let _ = std::io::stdin().read_line(&mut input_buffer);
    let _ = channel_tx.send(input_buffer.trim().to_owned());
    println!("Attempting to connect to: {}", input_buffer.trim());

    loop {
        std::thread::sleep(Duration::from_secs(50));
    }
}