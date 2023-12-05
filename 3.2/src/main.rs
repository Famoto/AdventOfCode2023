use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let sum = sum_multiplied_numbers(&contents);
    println!("Sum of multiplied numbers: {}", sum);
}

fn sum_multiplied_numbers(schematic: &str) -> i64 {
    let mut sum = 0;
    let lines: Vec<Vec<char>> = schematic.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '*' {
                println!("Found '*' at ({}, {})", x, y);
                let adjacent_numbers = find_adjacent_numbers(&lines, x, y, &mut visited);
                if adjacent_numbers.len() == 2 {
                    let product = adjacent_numbers[0] * adjacent_numbers[1];
                    println!("Multiplying {} and {} = {}", adjacent_numbers[0], adjacent_numbers[1], product);
                    sum += product;
                }
            }
        }
    }

    sum
}

fn find_adjacent_numbers(lines: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> Vec<i64> {
    let mut numbers = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let check_x = x as i32 + dx;
            let check_y = y as i32 + dy;
            if check_x >= 0 && check_y >= 0 && (check_x as usize) < lines[0].len() && (check_y as usize) < lines.len() {
                let new_x = check_x as usize;
                let new_y = check_y as usize;
                if lines[new_y][new_x].is_digit(10) {
                    let number = find_complete_number(lines, new_x, new_y, visited);
                    if !numbers.contains(&number) {
                        println!("Found number {} adjacent to '*' at ({}, {})", number, x, y);
                        numbers.push(number);
                    }
                }
            }
        }
    }
    numbers
}

fn find_complete_number(lines: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> i64 {
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
