use std::net::Ipv4Addr;

use wow_login_messages::all::{CMD_AUTH_LOGON_CHALLENGE_Client, Locale, Os, Platform, Version};
use wow_login_messages::ReadableAndWritable;

pub fn handler(username: &str) -> Vec<u8> {
    let builder = CMD_AUTH_LOGON_CHALLENGE_Client {
        protocol_version: 8,
        version: Version {
            major: 3,
            minor: 3,
            patch: 5,
            build: 12340,
        },
        platform: Platform::X86,
        os: Os::Windows,
        locale: Locale::EN_US,
        utc_timezone_offset: 180,
        client_ip_address: Ipv4Addr::new(127, 0, 0, 1).into(),
        account_name: username.to_uppercase(),
    };

    let mut packet: Vec<u8> = Vec::new();
    builder.write(&mut packet).unwrap();

    packet
}


fn main() {
    println!("Hello, world!");
}
