use fetch::Connection;

fn main() -> Result<(), std::io::Error> {
    let mut connection = Connection::new()?;

    println!("peer id {:x}", connection.peer_id());

    let peer = loop {
        connection.send_multicast_hello()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        connection.collect_responses()?;
        if let Some(peer) = connection.peers().first() {
            connection.send_multicast_hello()?;
            break *peer;
        }
        println!("searching…");
    };

    println!("connecting to {peer:?}");
    let connection = connection.connect(peer)?;

    println!("sending data…");

    loop {
        connection.send(b"hello")?;
        if let Some(msg) = connection.recv()? {
            print!("{}", String::from_utf8_lossy(&msg));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
