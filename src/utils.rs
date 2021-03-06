use std::io;

use interfaces::{Address, Interface};

pub fn get_all_addresses() -> io::Result<Vec<Address>> {
    Ok(Interface::get_all()
        .expect("Unable to get local IP addresses")
        .into_iter()
        .flat_map(|f| f.addresses.clone().into_iter())
        .collect())
}
