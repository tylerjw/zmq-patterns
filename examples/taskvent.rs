//! Task ventilator
//! Sends PUSH socket to tcp://localhost:5557
//! Sends batch of tasks to workers via that socket

use rand::Rng;
use zmq;

fn main() {
    let context = zmq::Context::new();
    let sender = context.socket(zmq::PUSH).unwrap();

    assert!(sender.bind("tcp://*:5557").is_ok());

    println!("Press Enter when the workers are ready:");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    println!("Sending the tasks to the workers...");

    // The first message is "0" and signals the start of a batch
    let sink = context.socket(zmq::PUSH).unwrap();
    assert!(sink.bind("tcp://*:5558").is_ok());
    let msg = zmq::Message::from("0");
    sink.send(msg, 0).unwrap();

    let mut rng = rand::rng();

    let mut total_msecs = 0;

    // Send 100 tasks
    for _ in 0..100 {
        // Random workload from 1 to 100msecs
        let workload = rng.random_range(1..=100);
        total_msecs += workload;

        let msg = zmq::Message::from(&format!("{workload}"));
        sender.send(msg, 0).unwrap();
    }

    println!("Total expected cost: {total_msecs} msec");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Done");
}