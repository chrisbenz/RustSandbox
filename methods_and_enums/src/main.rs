#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
}
