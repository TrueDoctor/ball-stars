use std::{
    io::Error,
    marker::PhantomData,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
};

const PORT: u16 = 9999;
const MADDR: Ipv4Addr = Ipv4Addr::new(239, 0, 0, 0);

pub struct Pairing;
pub struct Connected;

pub struct Connection<T> {
    peers: Vec<SocketAddr>,
    socket: UdpSocket,
    _state: PhantomData<T>,
}

impl Connection<Pairing> {
    pub fn new() -> Result<Connection<Pairing>, Error> {
        let socket = UdpSocket::bind((MADDR, PORT))?;
        socket.set_nonblocking(true)?;
        socket.join_multicast_v4(&MADDR, &Ipv4Addr::UNSPECIFIED)?;
        Ok(Self {
            peers: vec![],
            socket,
            _state: Default::default(),
        })
    }

    pub fn send_multicast_hello(&self) -> Result<(), Error> {
        self.socket.send_to(&42u32.to_le_bytes(), (MADDR, PORT))?;

        Ok(())
    }
    pub fn collect_responses(&mut self) -> Result<(), Error> {
        let mut buf = [0; 4];
        // todo deal with partial messages
        while let Ok((read_len, addr)) = self.socket.recv_from(&mut buf) {
            if read_len == 4 && u32::from_le_bytes(buf) == 42 && !self.peers.contains(&addr) {
                self.peers.push(addr);
            }
        }
        Ok(())
    }

    pub fn peers(&self) -> &[SocketAddr] {
        &self.peers
    }

    pub fn connect(self, addr: SocketAddr) -> Result<Connection<Connected>, Error> {
        let Self { peers, .. } = self;

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT))?;
        // socket.set_nonblocking(true)?;
        socket.connect(addr)?;
        Ok(Connection {
            peers,
            socket,
            _state: Default::default(),
        })
    }
}

impl Connection<Connected> {
    pub fn send(&self, message: &[u8]) -> Result<(), Error> {
        self.socket
            .send(message)
            .map(|len| assert_eq!(len, message.len(), "failed to send full message"))
    }

    pub fn recv(&self) -> Result<Vec<u8>, Error> {
        let mut len = 100;
        let mut output = Vec::new();
        let mut buf = vec![0; 100];
        while len == 100 {
            len = self.socket.recv(&mut buf)?;
            output.extend_from_slice(&buf[0..len]);
        }

        Ok(output)
    }
}
