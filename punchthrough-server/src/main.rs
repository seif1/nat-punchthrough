use std::net::{self, SocketAddr};
use std::collections::HashMap;
use rand::Rng;
use std::time;

fn main() {

    println!("\nAttempting to start server\n");

    let socket = net::UdpSocket::bind("0.0.0.0:8080").expect("Unable to bind");
    let _ = socket.set_nonblocking(true);

    println!("{socket:?}");

    let mut rng = rand::thread_rng();

    let mut connection_map: HashMap<u32, (SocketAddr, time::Instant)> = Default::default();
    let mut ip_dict: HashMap<SocketAddr, u32> = Default::default();


    let mut msg_buff: Vec<u8> = vec![0; 128];

    loop {
        if let Ok((_, addr)) = socket.recv_from(&mut msg_buff) {
            let message: Vec<&str> = std::str::from_utf8(&msg_buff).expect("Invalid Input").trim().trim_end_matches('\0').split(" ").collect();
            // Manage the possible message states
            match message[0] {
                // When a new peer connects to the server
                "START_CONNECTION" => {
                    let id: u32 = rng.gen();

                    if let Ok(_) = socket.send_to(format!("ID {:#X}", id).as_bytes(), addr) {
                        println!("Connection from: {} : {:#X}", addr, id);
                        connection_map.insert(id, (addr, time::Instant::now().checked_add(time::Duration::from_secs(3)).expect("Failed to add duration")));
                        ip_dict.insert(addr, id);
                    }
                },

                // Ensure connections are still active and waiting for a peer, and are not just sitting wasting resources
                "PONG" => {
                    if let Some(connection_id) = ip_dict.get(&addr) {
                        let entry = connection_map.entry(*connection_id);

                        entry.and_modify(|(_addr, instant)| {
                            *instant = time::Instant::now().checked_add(time::Duration::from_secs(3)).expect("Failed to add duration");
                        });
                    } else {
                        let _ = socket.send_to(b"ERROR:_IP_NOT_RECOGNIZED", addr);
                    }
                },

                // When a peer attempts to connect to another
                "CONNECT" => {
                    println!("Attempted Connection");
                    if let Ok(id) = u32::from_str_radix(&message[1][2..], 16) {
                        let mut connection_success = false;

                        if let Some((remote_addr, _)) = connection_map.get(&id) {
                            let _ = socket.send_to(format!("CONNECT {}", remote_addr).as_bytes(), addr);
                            let _ = socket.send_to(format!("CONNECT {}", addr).as_bytes(), remote_addr);
                            connection_success = true;
                        } else {
                            let _ = socket.send_to(b"UNKNOWN_ID", addr);
                        }

                        if connection_success {
                            connection_map.remove(&ip_dict.remove(&addr).unwrap());
                            ip_dict.remove(&connection_map.remove(&id).unwrap().0);
                        }

                    } else {
                        let _ = socket.send_to(b"INVALID_ID", addr);
                    }
                }
                _ => {
                    println!("Unknown message: {}", std::str::from_utf8(&msg_buff).expect("Invalid Input"));
                }
            }
        }

        // Manage Timeout values and PING active connections
        {
            connection_map.retain(|id, (addr, instant)| {
                if instant.elapsed() > time::Duration::ZERO {
                    println!("Removing: {} : {:#X}", addr, id);
                    let _ = ip_dict.remove(addr);
                    return false;
                } else {
                    let _ = socket.send_to(b"PING", *addr);
                    return true;
                }
            });
        }
        msg_buff.fill(0);
    }
}
