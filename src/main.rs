use std::net::{TcpStream, TcpListener};

fn main() {
    println!("Welcome to p2p-sharing");
    println!("Do you want to either connect to a port or create one?");
    println!("1: Connect");
    println!("2: Create");

    let mut option: String = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option = option.trim().to_string();

    match option.as_str() {
        "1" => {}
        "2" => {}
        _ => {
            println!("Invalid option specified!)")
        }
    }
}
