use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};

fn node() {
    let args: Vec<String> = std::env::args().collect();
    let chosen_port = &args[1];
    let addr = format!("127.0.0.1:{}", chosen_port);
    println!("Starting node on: {}", addr);
    let riv = TcpListener::bind(addr).unwrap();

    let mut phonebook: Vec<String> = Vec::new();
    for stream in riv.incoming() {
        let mut stream = stream.unwrap();

        // 2. Read what port they are calling from
        let mut buffer = [0; 128];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let incoming_node_port = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        // 3. Store it in a vector if it's a new port
        if !phonebook.contains(&incoming_node_port) && phonebook.len() < 5 {
            phonebook.push(incoming_node_port.clone());
        }

        // 4. mark of 5 store the node
        if phonebook.len() == 5 {
            // 5. Run the random index selector over the vector
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let random_index = (nanos % phonebook.len() as u128) as usize;
            let chosen_target_port = &phonebook[random_index];

            let target_addr = format!("127.0.0.1:{}", chosen_target_port);

            // 6. Execute the jump using TcpStream
            if let Ok(mut leap_stream) = TcpStream::connect(&target_addr) {
                // Tell the next node who we are so it can store US in its vector!
                let _ = leap_stream.write_all(chosen_port.as_bytes());
            // Break the loop or clear the phonebook to start over!
            break;
        }
    }
}

fn main() {
    node();
}
