// A function to get a number e.g. age or adding numbers
pub fn get_int() -> f64 {
    let mut line = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line: f64 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number");
                continue;
            }
        };
        return line as f64;
    }
}

pub fn adding() {
    println!("Tell me the first number to add");
    let first_num = get_int();
    println!("Tell me the second number to add");
    let second_num = get_int();
    let answer = first_num + second_num;
    println!("The sum of {first_num} and {second_num} is {answer}");
}
