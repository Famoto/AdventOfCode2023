use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let sum = sum_part_numbers(&contents);
    println!("Sum of part numbers: {}", sum);
}

fn sum_part_numbers(schematic: &str) -> i64 {
    let mut sum = 0;
    let lines: Vec<Vec<char>> = schematic.lines().map(|line| line.chars().collect()).collect();
    let mut visited = vec![vec![false; lines[0].len()]; lines.len()];

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x].is_digit(10) && !visited[y][x] {
                let number = extract_number(&lines, x, y, &mut visited);
                println!("Checking number {} at position ({}, {})", number, x, y);
                if number_adjacent_to_symbol(&lines, x, y) {
                    println!("Adding {} to sum", number);
                    sum += number;
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
    if check_x >= 0 && check_y >= 0 && (check_x as usize) < lines[0].len() && (check_y as usize) < lines.len() {
        let adj_char = lines[check_y as usize][check_x as usize];
        if adj_char != '.' && !adj_char.is_digit(10) {
            println!("Symbol {} found next to digit at ({}, {})", adj_char, x, y);
            return true;
        }
    }
    false
}
