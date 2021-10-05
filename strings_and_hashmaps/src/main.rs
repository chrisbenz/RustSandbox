fn main() {
    let mut str = String::new();

    let info = "Amazon Web Services";
    let info_s = info.to_string();
    let info_s2 =  "Statistics and Algorithms".to_string();
    let info_s3 = "Research";

    println!("{}, {}, and {}", &info_s, &info_s2, &info_s3);

    let shrug = String::from("¯\\(ツ)_/¯ ");

    println!("{}", &shrug);

    let mut fb = String::from("foo");
    fb.push_str("_bar");

    let mut team = String::from("Arizona Cardinal");
    team.push('s');
    println!("{}", &team);

    // Formatting complicated strings
    let field = String::from("State Farm Stadium");
    let loc = String::from(" Glendale, Arizona");
    let county = String::from(" Maricopa County");

    let full = format!("{},{},{}", field, loc, county);
    println!("{}", full);

    let hello = String::from("Здравствуйте");

    // Bytes to encode
    println!("Length is {}", hello.len());

    // chars
    for c in field.chars() {
        println!("{}", c);
    }

    // raw bytes
    for b in field.bytes() {
        println!("{}", b);
    }
}
