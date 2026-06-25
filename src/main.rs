use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn node() {
    let args: Vec<String> = std::env::args().collect();
    let chosen_port = &args[1];
    let addr = format!("127.0.0.1:{}", chosen_port);
    println!("Starting node on: {}", addr);
    let riv = TcpListener::bind(addr).unwrap();

    struct I2Ppacket {
        destination_port: String,
        message: String,
    }

    let packet = I2Ppacket {
        destination_port: String::from("9002"),
        message: String::from("hello from node 9001"),
    };

    // ONE SINGLE LOOP TO RULE THEM ALL
    for stream in riv.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established on port {}!", chosen_port);

        // 1. Reply back to whoever connected to us just to be nice
        let message_to_send = &packet.message;
        let _ = stream.write_all(message_to_send.as_bytes());
        println!("Sent packet message to the client!");

        // 2. Parse our port to math it out
        let current_port_num: u16 = chosen_port.parse().unwrap();

        // 3. Chain reaction: if we aren't at 9004, forward to the next one!
        if current_port_num < 9004 {
            let next_port_num = current_port_num + 1;
            let next_addr = format!("127.0.0.1:{}", next_port_num);

            println!("Forwarding packet automatically to next hop: {}", next_addr);

            if let Ok(mut next_node) = TcpStream::connect(&next_addr) {
                next_node.write_all(packet.message.as_bytes()).unwrap();
            } else {
                println!(
                    "Failed to connect to next hop {} - is it running?",
                    next_addr
                );
            }
        } else {
            println!(
                "We are the exit node (9004)! Packet destination reached: {}",
                packet.message
            );
        }
    }
}

fn main() {
    node();
}
