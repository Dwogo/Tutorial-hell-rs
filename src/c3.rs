pub fn higher_lower() {
    println!("Ok, now guess a number!");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let guess_num: i64 = line.trim().parse().unwrap();
    let secret_num: i64 = 10;

    match guess_num.cmp(&secret_num) {
        std::cmp::Ordering::Less => {
            println!("Too little");
        }
        std::cmp::Ordering::Equal => {
            println!("Perfect");
        }
        std::cmp::Ordering::Greater => {
            println!("Too Much");
        }
    }
}
