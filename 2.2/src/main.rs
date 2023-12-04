use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut sum_of_ids = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((id, game_data)) = parse_line(&line) {
            if is_game_possible(&game_data) {
                sum_of_ids += id;
            }
        }
    }

    println!("Sum of IDs: {}", sum_of_ids);
    Ok(())
}

fn parse_line(line: &str) -> Option<(i32, Vec<Vec<(i32, String)>>)> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        return None;
    }

    let id = parts[0].replace("Game ", "").parse::<i32>().ok()?;
    let sets = parts[1]
        .split(';')
        .map(|s| s.trim().split(',').map(|c| parse_cube(c.trim())).collect())
        .collect();

    Some((id, sets))
}

fn parse_cube(cube_str: &str) -> (i32, String) {
    let parts: Vec<&str> = cube_str.split_whitespace().collect();
    let number = parts[0].parse::<i32>().unwrap_or(0);
    let color = parts[1].to_string();

    (number, color)
}

fn is_game_possible(sets: &Vec<Vec<(i32, String)>>) -> bool {
    let max_cubes = [("red", 12), ("green", 13), ("blue", 14)];

    for set in sets {
        for (num, color) in set {
            if let Some(&(_, max)) = max_cubes.iter().find(|&&(c, _)| c == color) {
                if num > &max {
                    return false;
                }
            }
        }
    }

    true
}
