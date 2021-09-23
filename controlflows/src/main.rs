#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
    HalfDollar
}

fn main() {
    value_in_cents(Coin::Quarter(State::Arizona));
    let penny = value_in_cents(Coin::Penny);
    println!("{}", penny);

    // Rust has no nulls, instead an enum to encode a value being present or absent
    let missing_number: Option<i32> = None;
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is for the state of {:?}!", state);
            25
        },
        Coin::HalfDollar => 50
    }
}
