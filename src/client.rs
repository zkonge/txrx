use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    time::Duration,
};

use bytes::{Buf, BytesMut};

use crate::{url::Url, App};

fn get_available_stream(url: Url, args: &App) -> Option<TcpStream> {
    let mut rx_handshake_buffer = url.secret;
    for ip_to_try in dbg!(url.ips) {
        match TcpStream::connect_timeout(
            &SocketAddr::new(IpAddr::V4(ip_to_try), args.port),
            Duration::from_secs(1),
        ) {
            Ok(mut c) => {
                match c.write_all(&rx_handshake_buffer) {
                    Ok(_) => (),
                    Err(_) => continue,
                };
                c.set_read_timeout(Some(Duration::from_secs(1))).unwrap(); // It will fall?
                match c.read_exact(&mut rx_handshake_buffer) {
                    Ok(_) => {
                        if &rx_handshake_buffer == b"txrx" {
                            return Some(c);
                        } else {
                            continue;
                        }
                    }
                    Err(_) => continue,
                }
            }
            Err(_) => continue,
        };
    }
    None
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
