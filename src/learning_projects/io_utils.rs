use std::io;
use std::io::{BufRead, Write};

// Read number from stdin
pub fn read_number(prompt: &str) -> f64 {
    // Read the number from standard input
    let mut line = String::new();
    let num;
    loop {
        print!("{}", prompt);
        io::stdout().lock().flush().expect("Unable to flush stdout."); // Flush so line will print.

        io::stdin().lock().read_line(&mut line).expect("Error reading from stdin.");
        match line.trim().parse::<f64>() {
            Ok(n) => {
                num = n;
                break;
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                line.clear();
                continue;
            }
        };
    }
    return num;
}