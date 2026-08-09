use bitcode::{Decode, Encode};

use crate::game::Pos;

#[derive(Encode, Decode, Clone, Debug)]
pub struct Message {
    pub peer_id: u64,
    pub content: MessageType,
}

#[derive(Encode, Decode, Clone, Debug)]
pub enum MessageType {
    MousePosition(Pos),
}

pub enum Connection {
    Pairing(Option<fetch::Connection<fetch::Pairing>>),
    Connected(fetch::Connection<fetch::Connected>),
}

impl Connection {
    pub fn pair(&mut self) -> Option<()> {
        let Connection::Pairing(maybe_connection) = self else {
            return Some(());
        };
        let connection = maybe_connection.as_mut()?;
        connection.send_multicast_hello().ok()?;
        connection.collect_responses().ok()?;
        connection.send_multicast_hello().ok()?;
        let peer = *connection.peers().first()?;

        println!("\nconnecting to {peer:?}");
        let connection = maybe_connection.take().unwrap().connect(peer).ok()?;
        *self = Connection::Connected(connection);
        Some(())
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Connection::Connected(_))
    }

    pub fn fetch_updates(&self) -> Vec<Message> {
        let mut out = Vec::new();
        let Connection::Connected(connection) = self else {
            return out;
        };

        while let Ok(Some(res)) = connection.recv() {
            if let Ok(msg) = bitcode::decode(&res) {
                out.push(msg);
            }
        }
        out
    }
    pub fn send(&self, message: MessageType) {
        let Connection::Connected(connection) = self else {
            return;
        };

        let _ = connection.send(&bitcode::encode(&Message {
            peer_id: connection.peer_id(),
            content: message,
        }));
    }

    pub fn update(&mut self, pos: Pos) -> Option<Pos> {
        if !self.is_connected() {
            self.pair();
        } else {
        }
        match self {
            Connection::Pairing(maybe_connection) => {
                let connection = maybe_connection.as_mut()?;
                connection.send_multicast_hello().ok()?;
                connection.collect_responses().ok()?;
                connection.send_multicast_hello().ok()?;
                let peer = *connection.peers().first()?;

                println!("\nconnecting to {peer:?}");
                let connection = maybe_connection.take().unwrap().connect(peer).ok()?;
                *self = Connection::Connected(connection);
                None
            }
            Connection::Connected(connection) => {
                let mut message = [0; 16];
                message[0..8].copy_from_slice(&pos.0.to_ne_bytes());
                message[8..].copy_from_slice(&pos.1.to_ne_bytes());
                let _ = connection.send(&message);
                let mut other_pos = None;
                while let Ok(Some(res)) = connection.recv() {
                    if res.len() != message.len() {
                        continue;
                    }
                    let x = f64::from_le_bytes(res[0..8].try_into().unwrap());
                    let y = f64::from_le_bytes(res[8..].try_into().unwrap());
                    other_pos = Some((x, y));
                }
                other_pos
            }
        }
    }
}
