fn main() {
    let c = 'C';
    let b = 'B';
    let joy = '😂';
    let a = 'あ';

    println!("Different characters like {}, {}, {}, and {} are supported within Rust.", c, b, joy, a);

    let tup = (20000, 7.77, 23);
    let (x,y,z) = tup;
    println!("You can destructure tuples like so: {}, {}, and {}", x, y, z);

    let x: (&str, u8) = ("Sample Tuple", 1);
    let name: &str = x.0;
    let num: u8 = x.1;
    println!("{} and {}", name, num);

}
