use socketcan::{CANFrame, CANSocket};

fn dumpframe(f: &CANFrame) {
    print!("{:#x}: ", CANFrame::id(f));
    let data = CANFrame::data(f);
    println!("{:02x?}", data);
    
}

fn main() {
    let a = CANSocket::open("can0");
    match a {
        Ok(desc) => {
            loop {
                let frame = CANSocket::read_frame(&desc);
                match frame  {
                    Ok(f) => dumpframe(&f),
                    Err(_) => println!("Frame kaput"),
                }
            }
        }
        Err(_) => println!("Open failed!"),
    }
}
