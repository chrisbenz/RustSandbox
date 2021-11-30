use std::{env, task::Context, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // use std::env::args_os when invalid unicode is needed
    
    let query = &args[1];
    let filename = &args[2];

    println!("Query is {}", query);
    println!("Filename is {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong");

    println!("Contuseents are:\n{}", contents);
}
