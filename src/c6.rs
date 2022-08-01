pub fn menu() {
    println!("Here's what we serve: ");
    let menu = [
        "Steak",
        "Chicken",
        "Pork",
        "Fish",
        "Duck",
        "Cheese, just a block of cheese",
    ];

    for (x, y) in (0..menu.len()).enumerate() {
        let next = menu[x];
        let z = &y + 1;
        println!("{z}. {next}");
    }
}
