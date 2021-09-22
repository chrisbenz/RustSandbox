use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum IppAddrType {
    V4,
    V6
}

enum IppAddrType_Concise {
    V4(String),
    V6(String)
}

// Methods are defined within the context of a struct.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area is {}", rect1.area());
    let rect2 = (80, 100);
    println!("Area is {}", area2(&rect1));
    println!("{:#?}", &rect1);

    // Defining new enum types
    let four = IppAddrType::V4;
    let six = IppAddrType::V6;

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("{}", localhost_v4.is_ipv4());
    let concise_ip = IppAddrType_Concise::V4(String::from("::1"));
}
