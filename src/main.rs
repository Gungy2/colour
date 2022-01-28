use std::io::{stdin, stdout, Write};

fn main() {
    println!("Hello, Izabella! Welcome to guessing George's favourite colour!");

    loop {
        print!("Make your guess: ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string? How did u manage to fuck my game??");
        match s.trim().to_lowercase().as_str() {
            "blue" => {
                println!("Congratulations, now you know me!");
                break;
            }
            "pink" => println!("I AM NOT A GIRL!"),
            "green" => println!("Nature, ughhhh"),
            "red" => println!("Hot, but sadly no"),
            "yellow" => println!("Not a hippy either"),
            "black" => println!("I wish i were that cool?"),
            "white" => println!("Who in their right mind says their favourite colour is white, c'mon Izabella, you're better than this!"),
            _ => println!("Is this even a colour?? YOU ARE DISAPPOINTING ME!"),
        }
    }
}
