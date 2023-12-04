use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total_power = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((_, game_data)) = parse_line(&line) {
            let min_cubes = find_minimum_cubes(&game_data);
            let power = min_cubes.0 * min_cubes.1 * min_cubes.2;
            total_power += power;
        }
    }

    println!("Total power: {}", total_power);
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
fn find_minimum_cubes(sets: &Vec<Vec<(i32, String)>>) -> (i32, i32, i32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for set in sets {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (num, color) in set {
            match color.as_str() {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => {}
            }
        }

        min_red = min_red.max(red);
        min_green = min_green.max(green);
        min_blue = min_blue.max(blue);
    }

    (min_red, min_green, min_blue)
}
