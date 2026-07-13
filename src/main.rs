//! A simple Rust TCP Port Scanner
//! Author: Yassine Azmi
//! GitHub: https://github.com/YASSINE-azmi/TCP-Port-Scanner
//! This is a simple TCP port scanner written in Rust. It allows you to scan a range of ports on a specified IP address or hostname to check if they are open or closed.
//! This project is intended for educational purposes only. Please use it responsibly and only on networks you own or have permission to scan.
//! The project will be updated with new features and improvements in the future. If you have any suggestions or feedback, feel free to open an issue or submit a pull request on GitHub.
//! The project is licensed under the MIT License. See the LICENSE file for more details.
//! The project is for testing my knowledge in Rust programming language and networking concepts. It is not intended for any malicious use or unauthorized scanning of networks.
//! Thank you for using this project and happy scanning!
//! Version: 1.0.0

use std::net::TcpStream;
use std::time::Duration;

// Scans a single port on the given IP address and returns true if the port is open, false otherwise.
fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(1)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    // ip and port to scan
    let ip = "127.0.0.1";
    let port = 80;
    //function call to scan the port
    if scan_port(ip, port) {
        println!("Port {} is open", port);
    } else {
        println!("Port {} is closed", port);
    }
}
