use fetch::Connection;

fn main() -> Result<(), std::io::Error> {
    let mut connection = Connection::new()?;

    let peer = loop {
        connection.send_multicast_hello()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        connection.collect_responses()?;
        if !connection.peers().len() > 1 {
            break connection.peers()[1];
        }
    };

    let connection = connection.connect(peer)?;

    loop {
        connection.send(b"hello")?;
        let msg = connection.recv()?;
        print!("{}", std::str::from_utf8(&msg).unwrap());
    }
}
