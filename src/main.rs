pub mod c1;
pub mod c2;
pub mod c3;
pub mod c4;
pub mod c5;
pub mod c6;

fn main() {
    // Welcome message
    println!("Hello CBHS");

    loop {
        println!("  ");
        println!("Which chapter do you want to use (Use the chapter number or nothing for the whole program)");

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let chapter: String = line.trim().parse().unwrap();

        if chapter == "1" {
            c1::get_name();
        } else if chapter == "2" {
            c2::adding();
        } else if chapter == "3" {
            c3::higher_lower();
        } else if chapter == "4" {
            c4::color();
        } else if chapter == "5" {
            c5::for_loop();
        } else if chapter == "6" {
            c6::menu();
        } else {
            c1::get_name();
            println!("  ");
            c2::adding();
            println!("  ");
            c3::higher_lower();
            println!("  ");
            c4::color();
            println!("  ");
            c5::for_loop();
            println!("  ");
            c6::menu();
        }
        println!("Do you want to continue?");

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let yes_or_no: String = line.trim().parse().unwrap();
        let yes_or_no: String = yes_or_no.to_ascii_lowercase();

        if yes_or_no == "y" || yes_or_no == "yes" {
            continue;
        } else {
            break;
        }
    }
}
