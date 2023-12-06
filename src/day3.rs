use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_owned()
}

// Part 1: Sum of part numbers
#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];
    let mut sum = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x].is_digit(10) && !visited[y][x] {
                let number = extract_number(&lines, x, y, &mut visited);
                if number_adjacent_to_symbol(&lines, x, y) {
                    sum += number;
                }
            }
        }
    }
    sum
}

// Part 2: Sum of multiplied numbers
#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];
    let mut sum = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '*' {
                let adjacent_numbers = find_adjacent_numbers(&lines, x, y, &mut visited);
                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }
    sum
}

fn extract_number(lines: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> i64 {
    let mut num_str = String::new();
    let mut x = x;
    while x < lines[y].len() && lines[y][x].is_digit(10) {
        num_str.push(lines[y][x]);
        visited[y][x] = true;
        x += 1;
    }
    num_str.parse::<i64>().unwrap_or(0)
}

fn number_adjacent_to_symbol(lines: &Vec<Vec<char>>, start_x: usize, y: usize) -> bool {
    let mut x = start_x;
    while x < lines[y].len() && lines[y][x].is_digit(10) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if is_adjacent(lines, x, y, dx, dy) {
                    return true;
                }
            }
        }
        x += 1;
    }
    false
}

fn is_adjacent(lines: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let check_x = x as i32 + dx;
    let check_y = y as i32 + dy;
    if check_x >= 0
        && check_y >= 0
        && (check_x as usize) < lines[0].len()
        && (check_y as usize) < lines.len()
    {
        let adj_char = lines[check_y as usize][check_x as usize];
        if adj_char != '.' && !adj_char.is_digit(10) {
            return true;
        }
    }
    false
}

fn sum_multiplied_numbers(schematic: &str) -> i64 {
    let mut sum = 0;
    let lines: Vec<Vec<char>> = schematic
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '*' {
                let adjacent_numbers = find_adjacent_numbers(&lines, x, y, &mut visited);
                if adjacent_numbers.len() == 2 {
                    let product = adjacent_numbers[0] * adjacent_numbers[1];
                    sum += product;
                }
            }
        }
    }
    sum
}

fn find_adjacent_numbers(
    lines: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<i64> {
    let mut numbers = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            if check_x >= 0
                && check_y >= 0
                && (check_x as usize) < lines[0].len()
                && (check_y as usize) < lines.len()
            {
                let new_x = check_x as usize;
                let new_y = check_y as usize;
                if lines[new_y][new_x].is_digit(10) {
                    let number = find_complete_number(lines, new_x, new_y, visited);
                    if !numbers.contains(&number) {
                        numbers.push(number);
                    }
                }
            }
        }
    }
    numbers
}

fn find_complete_number(
    lines: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
) -> i64 {
    // Search left for the start of the number
    let mut start_x = x;
    while start_x > 0 && lines[y][start_x - 1].is_digit(10) {
        start_x -= 1;
    }

    // Now extract the complete number
    let mut num_str = String::new();
    let mut current_x = start_x;
    while current_x < lines[y].len() && lines[y][current_x].is_digit(10) {
        num_str.push(lines[y][current_x]);
        visited[y][current_x] = true;
        current_x += 1;
    }
    num_str.parse::<i64>().unwrap_or(0)
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 467835);
    }
}
