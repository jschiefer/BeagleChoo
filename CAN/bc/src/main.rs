use socketcan::CANSocket;

fn main() {
    let a = CANSocket::open("can0");
    match a {
        Ok(_) => println!("Open successful!"), 
        Err(_) => println!("Open failed!")
    }
}
