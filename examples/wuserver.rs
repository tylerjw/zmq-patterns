//! Weather update server
//! Binds PUB socket to tcp://*:5556
//! Publishes random weather updates
use zmq;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());

    loop {
        // Get values that will fool the boss
        let zipcode = rand::random::<u32>() % 100000;
        let temperature = rand::random::<i16>() % 215 - 80;
        let relhumidity = rand::random::<i16>() % 50 + 10;

        // Send message to all subscribers
        let message = format!("{zipcode:05} {temperature} {relhumidity}");
        publisher.send(message.as_bytes(), 0).unwrap();
    }
}
