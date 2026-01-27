//! Print the version of the ZeroMQ library
use zmq;

fn main() {
    let (major, minor, patch) = zmq::version();
    println!("ZeroMQ version: {}.{}.{}", major, minor, patch);
}
