pub fn for_loop() {
    println!("Okay, now choose another positive number (it's not another high/lower game)");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let line: u64 = line.trim().parse().unwrap();

    let how_long = line + 1;

    for x in 1..how_long {
        println!("{x}");
    }
}
