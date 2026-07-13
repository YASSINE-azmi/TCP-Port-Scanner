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
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};


// Scans a single port on the given IP address and returns true if the port is open, false otherwise.
fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    match TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(1)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

//spawn a thread for each port to scan concurrently & sync the results using channels
fn scan_ports_concurrently(ip: &str, start_port: u16, end_port: u16) {
    let (tx, rx): (Sender<(u16, bool)>, Receiver<(u16, bool)>) = mpsc::channel();

    for port in start_port..=end_port {
        let tx = tx.clone();
        let ip = ip.to_string();
        thread::spawn(move || {
            let is_open = scan_port(&ip, port);
            tx.send((port, is_open)).unwrap();
        });
    }

    drop(tx); // Close the sending side of the channel

    for (port, is_open) in rx {
        if is_open {
            println!("Port {} is open", port);
        } else {
            println!("Port {} is closed", port);
        }
    }
}

fn main() {
    // ip and port to scan
    let ip = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;
    scan_ports_concurrently(ip, start_port, end_port);
}
