struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    slices();
    structs();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn slices() {
    let s = String::from("Arizona Diamondbacks");
    let res = first_word(&s);
    println!("{}", res);

    let team = &s[8..s.len()];
    let team2 = &s[8..];
    assert_eq!(team, team2);
    println!("{} and {}", team, team2);
}

fn structs() {
    let user1 = User {
        username: String::from("randomUser1"),
        email: String::from("randomuser1@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    // The entire struct must be marked as mutable in Rust.
    let mut user2_mut = User {
        username: String::from("randomUser2"),
        email: String::from("randomuser2@gmail.com"),
        ..user1
    };

    user2_mut.email = String::from("randomuser2_alternate@gmail.com");

    println!("{}", user1.username);
    println!("{}", user1.sign_in_count);
    println!("{}", user2_mut.email);
    println!("{}" , user2_mut.active);
    assert_eq!(user1.active, user2_mut.active);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let color1 = Color(255,255,255);
    let point1 = Point(1, 25, 50);

    
    let user_from_function = get_new_user(String::from("joesmith@gmail.com"), String::from("jsmith1"));
    println!("{}", user_from_function.email);
}

fn get_new_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}