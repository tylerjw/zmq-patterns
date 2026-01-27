//! Weather server client
//! Connects SUB socket to tcp://localhost:5556
//! Collects weather updates and finds average temp in zipcode
use zmq;

fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();

    assert!(subscriber.connect("tcp://localhost:5556").is_ok());

    // Subscribe to zipcode, default is NYC, 10001
    let zipcode = b"10001";
    subscriber.set_subscribe(zipcode).unwrap();

    let mut total_temp = 0;

    // Process 100 updates
    let n_updates = 100;
    for update_nbr in 0..n_updates {
        let message = subscriber.recv_string(0).unwrap().unwrap();
        let parts: Vec<&str> = message.split(' ').collect();
        let zipcode = parts[0].parse::<u32>().unwrap();
        let temperature = parts[1].parse::<i16>().unwrap();
        let relhumidity = parts[2].parse::<i16>().unwrap();

        println!(
            "Received update {update_nbr}: zipcode {}, temperature {}, humidity {}",
            zipcode, temperature, relhumidity
        );
        total_temp += temperature;
    }

    println!(
        "Average temperature for zipcode {} was {}",
        str::from_utf8(zipcode).unwrap(),
        total_temp / n_updates
    );
}
