use std::{io::{self, Write}, process::exit};

fn main() {
    print!("From (f/c): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut source_type = String::new();
    io::stdin().
        read_line(&mut source_type).
        expect("Failed to read line");

    let source_type = source_type.trim().to_ascii_lowercase();
    if source_type != "f" && source_type != "c" {
        println!("Temperature type unsupported");
        exit(1);
    }

    print!("Temperature: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("Please input a number!");

    if source_type == "f" {
        let conv = 5.0*(temp-32.0)/9.0;
        println!("{:.2}°f is {:.2}°c", temp, conv)
    } else if source_type == "c" {
        let conv = temp*9.0/5.0+32.0;
        println!("{:.2}°c is {:.2}°f", temp, conv)
    }
}
