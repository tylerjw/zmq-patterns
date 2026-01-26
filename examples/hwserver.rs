//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"
use zmq;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1));
        responder.send("Wrold", 0).unwrap();
    }
}
