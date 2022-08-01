pub fn color() {
    println!("What's your favourite CBHS colour?");

    // Input
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let usr_color: String = line.trim().parse().unwrap();
    let usr_color: String = usr_color.split_ascii_whitespace().collect();

    if &usr_color.to_ascii_lowercase() == "blue"
        || &usr_color.to_ascii_lowercase() == "black"
        || &usr_color.to_ascii_lowercase() == "gold"
        || &usr_color.to_ascii_lowercase() == "white"
    {
        println!("{usr_color}, CBHS Colours!")
    } else {
        println!("Wait a minute {usr_color}, that's not a CBHS colour");
    }
}
