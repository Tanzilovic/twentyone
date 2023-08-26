use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut counter = 0;

    loop {
        println!("(21 Dares) Enter your number:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read number!");

        let input: u32 = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid type given!");
                continue;
            }
        };

        counter += input;

        println!("Current number: {counter}");

        if counter == 20 {
            print!("\x1B[2J\x1B[1;1H");
            println!("Current number: {counter}");
            thread::sleep(Duration::from_secs(1));
            println!("You win!");
            thread::sleep(Duration::from_secs(4));
            break;
        }
    }
}