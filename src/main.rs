mod planetary_system_generator;
mod star_generator;
mod planet_generator;

use std::io;

fn main() {
    println!("------------------------");
    println!("A S T R O B I O L O G Y");
    println!("------------------------");
    println!();
    println!("Press ENTER to explore a system. Type QUIT and ENTER to end game.");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match &input.trim() as &str {
            "QUIT" => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("{}", planetary_system_generator::explore_system());
            }
        }
    }
}