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

    let some_u8_val = Some(0u8);
    if let Some(3) = some_u8_val {
        println!("Val is equal to three");
    }

    let mut count = 0;

    let some_coin = Coin::Quarter(State::Arizona);
    
    // match some_coin {
    //     Coin::Quarter(state) => println!("State quarter from the {:?} state", state),
    //     _ => count += 1,
    // }

    // Same thing as above but with if let else
    if let Coin::Quarter(state) = some_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
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
