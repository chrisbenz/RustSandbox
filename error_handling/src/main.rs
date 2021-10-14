use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    println!("Hello, world!");
    // panic!("Program crash");

    // Would crash the program if a run of this program was attempted 
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("notes.txt");
    
    // Checking the kind of error to determine if a file should be created
    let f = match f {
        Ok(file) => file,
        Err(error) => match(error.kind()) {
            ErrorKind::NotFound => match File::create("notes.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create file {:?}", e),
            },
            other_error => {
                panic!("An error occurred while attepmting to open the file {:?}", other_error)
            }
        }
    };

    // More succinct ways of eventaully calling panic!
    // let b = File::open("abc.txt").unwrap();
    // let f2 = File::open("hello.txt").expect("Failed to open hello.txt");

    let res:Result<String, io::Error> = read_username_from_file();
    println!("{:?}", res);

    match res {
        Ok(_) => Ok("Success"),
        Err(e) => Err(e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}