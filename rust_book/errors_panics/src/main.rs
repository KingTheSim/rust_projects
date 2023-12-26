use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
}