use std::io::Write;

use fetch::Connection;

fn main() -> Result<(), std::io::Error> {
    let mut connection = Connection::new()?;

    println!("peer id {:x}", connection.peer_id());

    print!("searching…");
    let peer = loop {
        connection.send_multicast_hello()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        connection.collect_responses()?;
        if let Some(peer) = connection.peers().first() {
            connection.send_multicast_hello()?;
            break *peer;
        }
        print!("…");
        std::io::stdout().flush()?;
    };

    println!("\nconnecting to {peer:?}");
    let connection = connection.connect(peer)?;

    println!("sending data…");

    loop {
        connection.send(b"hello")?;
        if let Some(msg) = connection.recv()? {
            println!("{}", String::from_utf8_lossy(&msg));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
