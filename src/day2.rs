
#[aoc_generator(day2)]
fn parse(input: &str) -> String {
    input.to_owned()
}
#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {

    let mut sum_of_ids = 0;

    for line in input.lines() {
        if let Some((id, game_data)) = parse_line(&line) {
            if is_game_possible(&game_data) {
                sum_of_ids += id;
            }
        }
    }

    //println!("Sum of IDs: {}", sum_of_ids);
    sum_of_ids
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

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {

    let mut total_power = 0;

    for line in input.lines() {
        if let Some((_, game_data)) = parse_line(&line) {
            let min_cubes = find_minimum_cubes(&game_data);
            let power = min_cubes.0 * min_cubes.1 * min_cubes.2;
            total_power += power;
        }
    }

    //println!("Total power: {}", total_power);
    total_power
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


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 2286);
    }
}