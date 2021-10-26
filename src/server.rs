use std::{
    fs::File,
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener},
    path::Path,
};

use bytes::{BufMut, BytesMut};
use ifaces::{ifaces, Kind};

use crate::{url::Url, App};

fn get_valid_ips() -> io::Result<Vec<Ipv4Addr>> {
    //TODO: IPv6? Coming soon...
    let interfaces = ifaces()?
        .into_iter()
        .filter_map(|x| match (x.kind, x.addr) {
            (Kind::Ipv4, Some(SocketAddr::V4(x))) => Some(*x.ip()),
            _ => None,
        })
        .collect();
    Ok(interfaces)
}

pub fn server(args: App) {
    // 1. Initialization
    let filepath = Path::new(&args.source);
    assert!(!filepath.is_dir(), "Send directory is currently not supported");

    let ips = get_valid_ips().expect("Unable to detect available IPs");
    let mut file = File::open(filepath).unwrap();
    let url = Url::new(&ips);
    println!("Here is your connection token: {}", url.to_string());

    // 2. Handshake
    let listener = TcpListener::bind(("0.0.0.0", args.port)).unwrap();
    let (mut socket, addr) = listener.accept().unwrap();
    println!("Get connection from {}", addr);

    let mut buffer = [0u8; 4];

    socket.read_exact(&mut buffer).unwrap();
    assert!(buffer == url.secret, "Handshake failed, wrong secret");
    socket.write_all(b"txrx").unwrap();

    // 3. Exchange metadata (filename, filesize)
    // Protocol: [2 bytes filename length][filename][8 bytes file size]
    let mut metadata_buffer = BytesMut::new();
    let filename = filepath.file_name().unwrap().to_str().unwrap();
    metadata_buffer.put_u16(filename.len() as u16); // Filename longer than 65535 is impossible
    metadata_buffer.put_slice(filename.as_bytes());
    metadata_buffer.put_u64(file.metadata().expect("Unable to detect file size").len());
    socket.write_all(&metadata_buffer).unwrap();

    // 4. Transfer file
    io::copy(&mut file, &mut socket).unwrap();
}
