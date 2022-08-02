pub fn higher_lower() {
    println!("Ok, now guess a number!");

    loop {
        // User input
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let guess_num: i64 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number");
                continue;
            }
        };

        // The secret number
        let secret_num: i64 = 10;

        match guess_num.cmp(&secret_num) {
            std::cmp::Ordering::Less => {
                println!("Too little");
                break;
            }
            std::cmp::Ordering::Equal => {
                println!("Perfect");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too Much");
                break;
            }
        }
    }
}
