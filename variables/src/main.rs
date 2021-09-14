fn main() {

    // Variables and Mutability
    let mut x = 5;
    println!("X value = {}", x);
    x = 6;
    println!("Value of X is now {}", x);

    const MAX_POINTS: u32 = 100_000;
    const NAME: &str = "Chris";

    println!("Max points for the full iteration have been assigned as {}", MAX_POINTS);
    println!("Sign your name here: {}", NAME);

    // Shadowing
    let x = 10;
    let x = x + 20;
    let x = x * x;

    println!("The value of x is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();

    println!("There are {} spaces", spaces);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Length of slice is {}", slice.len());
    println!("Contents of slice are {:?}", slice);

    let another_slice = &a[0..4];
    println!("Contents of this slice are {:?}", another_slice);
}
