pub fn get_name() {
    // Getting the name of the user
    println!("What is your name?");

    let mut usr_name = String::new();
    std::io::stdin()
        .read_line(&mut usr_name)
        .expect("Failed to read line");
    let usr_name: String = usr_name.trim().parse().unwrap();

    // Also if the user's name is "Mr T" it displays a special message
    if &usr_name == "Mr T" {
        println!("I pity the fool");
    } else {
        println!("Hello, {usr_name}");
    }
}
