use std::net::TcpListener;

fn node() {
    let args: Vec<String> = std::env::args().collect();
    let chosen_port = &args[1];
    let addr = format!("127.0.0.1:{}", chosen_port);
    println!("Starting node on: {}", addr);
    let riv = TcpListener::bind(addr).unwrap();

    for stream in riv.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established! Someone just pinged our server!");
    }
}
fn main() {
    node();
}
