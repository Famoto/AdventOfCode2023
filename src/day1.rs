#[aoc_generator(day1)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day1, part1)]
fn day1_part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        if let Some(first_digit) = line.chars().find(|c| c.is_digit(10)) {
            if let Some(last_digit) = line.chars().rev().find(|c| c.is_digit(10)) {
                let number_str = format!("{}{}", first_digit, last_digit);
                if let Ok(number) = number_str.parse::<u32>() {
                    sum += number;
                }
            }
        }
    }
    sum
}

#[aoc(day1, part2)]
fn day1_part2(input: &str) -> u32 {
    let mut sum = 0;
    //println!("Sum: {sum}");
    for line in input.lines() {
        //println!("Original line: {}", line);
        //Replace each digit word with its corresponding digit
        let newline = line
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

        //println!("Transformed line: {}", newline);

        let digits: Vec<char> = newline.chars().filter(|c| c.is_digit(10)).collect();
        //println!("Digits found: {:?}", digits);

        if let (Some(&first_digit), Some(&last_digit)) = (digits.first(), digits.last()) {
            let number_str = format!("{}{}", first_digit, last_digit);
            //println!("Concatenated number: {}", number_str);
            if let Ok(number) = number_str.parse::<u32>() {
                sum += number;
                //println!("Sum: {sum}");
            }
        }
    }
    sum
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        assert_eq!(day1_part1(SAMPLE), 142);
    }

    #[test]
    fn part2_example() {
        const SAMPLE: &str = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        assert_eq!(day1_part2(SAMPLE), 281);
    }
}
