use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    sync::mpsc::channel,
    thread::spawn,
    time::Duration,
};

use bytes::{Buf, BytesMut};

use crate::{url::Url, App};

fn get_available_stream(url: Url, args: &App) -> Option<TcpStream> {
    let mut rx_handshake_buffer = url.secret;

    let (tx, rx) = channel::<TcpStream>();

    dbg!(url.ips).iter().map(|x| IpAddr::V4(*x)).for_each(|ip| {
        let tx = tx.clone();
        let port = args.port;
        spawn(move || {
            if let Ok(mut c) = TcpStream::connect_timeout(&SocketAddr::new(ip, port), Duration::from_secs(1)) {
                if c.write_all(&rx_handshake_buffer).is_err() {
                    return;
                }
                c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                if c.read_exact(&mut rx_handshake_buffer).is_err() {
                    return;
                };
                if &rx_handshake_buffer == b"txrx" {
                    tx.send(c).unwrap();
                }
            };
        });
    });
    rx.recv_timeout(Duration::from_secs(5)).ok()
}

pub fn client(args: App) {
    // 1. Initialization
    let url = Url::from_str(&args.source).expect("Wrong format");

    // 2. Handshake
    let mut connection = get_available_stream(url, &args).expect("Unable to connect source");

    let mut metadata_buffer = BytesMut::new();

    metadata_buffer.resize(2, 0);
    connection
        .read_exact(&mut metadata_buffer)
        .expect("Get filename size failed");

    let filename_size = dbg!(metadata_buffer.get_u16() as usize);

    metadata_buffer.resize(filename_size, 0);
    connection
        .read_exact(&mut metadata_buffer)
        .expect("Get filename failed");
    let filename = dbg!(metadata_buffer.copy_to_bytes(filename_size));

    metadata_buffer.resize(8, 0);
    connection.read_exact(&mut metadata_buffer).expect("Get size failed");
    let size = dbg!(metadata_buffer.get_u64());

    // 3. Create file
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&*String::from_utf8_lossy(&filename))
        .expect("Unable to create file");

    // 4. Transfer
    let length = io::copy(&mut connection, &mut file).expect("Broken file");

    if length != size {
        eprintln!("File transfer not complete, try again?");
    }
}
