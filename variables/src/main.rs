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

}
