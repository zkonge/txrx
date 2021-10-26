#![deny(unsafe_code)]

use clap::Parser;

mod client;
mod server;
mod url;
#[cfg(not(target_os = "windows"))]
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(name = "txrx")]
pub struct App {
    /// Number of times to greet
    #[clap(short, long, default_value = "11363")]
    port: u16,

    /// Two choices:
    /// 1. File to send (as sender)
    /// 2. URL like "txrx://xxxxx" (as receiver)
    source: String,
}

fn main() {
    let args: App = App::parse();
    if args.source.starts_with(url::TXRX_SCHEME) {
        client::client(args);
    } else {
        server::server(args);
    }
}
