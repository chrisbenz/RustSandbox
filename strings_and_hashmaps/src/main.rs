use std::collections::HashMap;
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

    hashmaps();
}

fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Blue"), 40); 

    let teams = vec![String::from("New York Giants"), String::from("New England Patriots")];
    let final_scores = vec![13, 20];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(final_scores.into_iter()).collect();
    println!("{:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", &map);

    // No longer valid at this point.
    // println!("{}", field_name);

    for (key, value) in &scores2 {
        println!("{}, {}", key, value);
    }

    // Overwrite
    scores.insert(String::from("Red"), 25);
    scores.insert(String::from("Red"), 35);
    scores.insert(String::from("Red"), 45);

    println!("{:?}", scores);
    
    // Insert if key has no value
    scores.entry(String::from("Red")).or_insert(99);
    scores.entry(String::from("Orange")).or_insert(1111);
    println!("{:?}", scores);

    // Update using an old value
    let text = "Java, Rust, C++, Haskell";

    let mut mapData = HashMap::new();

    for word in text.split_whitespace() {
        let count = mapData.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", mapData);

}
