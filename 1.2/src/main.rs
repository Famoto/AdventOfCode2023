use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt"; // Replace with your file path
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    println!("Sum: {sum}");
    for line in reader.lines() {
        let mut line = line?;
        println!("Original line: {}", line);

        //Replace each digit word with its corresponding digit
        line = line
            .replace("zero", "0ero")
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "thr3e")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "s7ven")
            .replace("eight", "e8ght")
            .replace("nine", "n9ne");

        println!("Transformed line: {}", line);

        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        println!("Digits found: {:?}", digits);

        if let (Some(&first_digit), Some(&last_digit)) = (digits.first(), digits.last()) {
            let number_str = format!("{}{}", first_digit, last_digit);
            println!("Concatenated number: {}", number_str);
            if let Ok(number) = number_str.parse::<i32>() {
                sum += number;
                println!("Sum: {sum}");
            }
        }
    }

    println!("Sum of numbers: {}", sum);
    Ok(())
}
