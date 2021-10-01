fn main() {

    // A new Vec<T> to hold elements of the i32 type
    let _vector: Vec<i32> = Vec::new();

    // Also possible for Rust to infer the type of value stored based on elements inserted
    let v = vec![1, 2, 3, 10, 99, 129, 8213];
    println!("{:?}", v);

    let mut v = Vec::new();

    v.push(11111);
    v.push(22222);
    v.push(33333);
    v.push(44444);

    println!("Mutable vector consists of {:?}", v);

    let newVector = vec![1, 2, 3, 4, 5];

    let fourth: &i32 = &newVector[3];
    println!("Fourth element in the vector is {}", fourth);

    match newVector.get(3) {
        Some(fourth) => println!("Fourth element in the vector is {}", fourth),
        None => println!("No fourth element"),
    }

} // v would go out of scope at this point and be freed
