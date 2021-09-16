fn main() {
    // println!("{}", res)

    let mut s = String::from("Arizona Diamondbacks");
    let res = first_word(&s);
    println!("{}", res);

    let team = &s[8..s.len()];
    let team2 = &s[8..];
    assert_eq!(team, team2);
    println!("{} and {}", team, team2);

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