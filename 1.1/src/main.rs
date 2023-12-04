use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let mut sum = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if let Some(first_digit) = line.chars().find(|c| c.is_digit(10)) {
            if let Some(last_digit) = line.chars().rev().find(|c| c.is_digit(10)) {
                let number_str = format!("{}{}", first_digit, last_digit);
                if let Ok(number) = number_str.parse::<i32>() {
                    sum += number;
                }
            }
        }
    }

    println!("Sum of numbers: {}", sum);
    Ok(())
}
