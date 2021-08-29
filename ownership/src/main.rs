fn main() {
    test();

    // Mutable Strings
    let mut s = String::from("hello");
    println!("{}", s);

    // Append a string literal
    s.push_str(" world");
    println!("{}", s);

    let s1 = String::from("Test");
    // Deep copies
    let s2 = s1.clone();
    println!("{} , {}!", s1, s2);

    // Integers are stored entirely on the stack.
    let first = 1;
    let second = 2;
    println!("{} and {}", first, second)

    
}

fn test() {
    println!("Hi")
}
