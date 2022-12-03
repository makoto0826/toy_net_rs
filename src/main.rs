use std::{io::Read, net::SocketAddr, time::Duration};

use socket2::{Domain, Protocol, Socket, Type};
use toy_net_rs::{
    errors::Error,
    icmp::{EchoReply, EchoRequest, ALPHABET},
    ip::InternetProtocolV4,
};

fn main() {
    ping();
}

fn ping() -> Result<(), Error> {
    let dest = SocketAddr::new("127.0.0.1".parse().unwrap(), 0);
    let mut socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    let mut buffer: [u8; 8 + 64] = [0; 8 + 64];
    let mut payload: [u8; 64] = [0; 64];

    for i in 0..payload.len() {
        payload[i] = ALPHABET[i % ALPHABET.len()];
    }

    let request = EchoRequest {
        sequence: 33,
        identity: 1000,
        payload: &payload,
    };

    request.encode(&mut buffer)?;

    socket.set_ttl(128)?;
    socket.set_write_timeout(Some(Duration::from_secs(1)))?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    socket.send_to(&buffer, &dest.into())?;

    let mut buffer: [u8; 2048] = [0; 2048];
    let size = socket.read(&mut buffer)?;
    let ip = InternetProtocolV4::decode(&buffer, size);

    EchoReply::decode(&ip.data)?;

    println!("OK");
    Ok(())
}
