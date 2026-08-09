use std::{
    hash::{BuildHasher, Hasher, RandomState},
    io::{Error, ErrorKind},
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
};

const PORT: u16 = 9876;
const MADDR: Ipv4Addr = Ipv4Addr::new(239, 0, 0, 123);
const MAGIC: u32 = 42;
const HELLO_LEN: usize = 12;
const MAX_DATAGRAM: usize = 65_536;

pub struct Pairing;
pub struct Connected;

pub struct Connection<T> {
    peers: Vec<SocketAddr>,
    socket: UdpSocket,
    peer_id: u64,
    _state: PhantomData<T>,
}

impl Connection<Pairing> {
    pub fn new() -> Result<Connection<Pairing>, Error> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT))?;
        socket.set_nonblocking(true)?;
        socket.join_multicast_v4(&MADDR, &Ipv4Addr::UNSPECIFIED)?;
        Ok(Self {
            peers: vec![],
            socket,
            peer_id: RandomState::new().build_hasher().finish(),
            _state: Default::default(),
        })
    }

    pub fn send_multicast_hello(&self) -> Result<(), Error> {
        let mut hello = [0; HELLO_LEN];
        hello[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        hello[4..].copy_from_slice(&self.peer_id.to_le_bytes());
        self.socket.send_to(&hello, (MADDR, PORT))?;

        Ok(())
    }

    pub fn collect_responses(&mut self) -> Result<(), Error> {
        let mut buf = [0; HELLO_LEN];
        loop {
            let (read_len, addr) = match self.socket.recv_from(&mut buf) {
                Ok(v) => v,
                Err(e) if e.kind() == ErrorKind::WouldBlock => return Ok(()),
                Err(e) => return Err(e),
            };
            if read_len != HELLO_LEN {
                continue;
            }
            let magic = u32::from_le_bytes(buf[0..4].try_into().unwrap());
            let sender_id = u64::from_le_bytes(buf[4..].try_into().unwrap());
            if magic == MAGIC && sender_id != self.peer_id && !self.peers.contains(&addr) {
                self.peers.push(addr);
            }
        }
    }

    pub fn connect(self, addr: SocketAddr) -> Result<Connection<Connected>, Error> {
        let Self {
            peers,
            socket,
            peer_id,
            ..
        } = self;

        socket.leave_multicast_v4(&MADDR, &Ipv4Addr::UNSPECIFIED)?;
        socket.connect(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Connection {
            peers,
            socket,
            peer_id,
            _state: Default::default(),
        })
    }
}

impl<T> Connection<T> {
    pub fn peer_id(&self) -> u64 {
        self.peer_id
    }
    pub fn peers(&self) -> &[SocketAddr] {
        &self.peers
    }
}

impl Connection<Connected> {
    pub fn send(&self, message: &[u8]) -> Result<(), Error> {
        self.socket
            .send(message)
            .map(|len| assert_eq!(len, message.len(), "failed to send full message"))
    }

    pub fn recv(&self) -> Result<Option<Vec<u8>>, Error> {
        let mut buf = vec![0; MAX_DATAGRAM];
        match self.socket.recv(&mut buf) {
            Ok(len) => {
                buf.truncate(len);
                Ok(Some(buf))
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }
}
