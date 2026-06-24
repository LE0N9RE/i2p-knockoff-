use std::net::TcpListener;
fn main() {
    //listen for incoming connections
    let net = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in net.incoming() {
        // grab the connection.
        let _stream = stream.unwrap();
        println!("Connection established! Someone just pinged our server!");
    }
}
