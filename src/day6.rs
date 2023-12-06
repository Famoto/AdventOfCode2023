use aoc_runner_derive::{aoc, aoc_generator};
// Define a structure to hold the race details
#[derive(Default)]
struct Race {
    time: i64,
    record_distance: i64,
}

// Parse the input into a vector of Race structs
#[aoc_generator(day6, part1)]
fn parse_input(input: &str) -> tinyvec::ArrayVec<[Race; 4]> {
    let mut lines = input.lines();

    let times: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let distances: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}
#[aoc_generator(day6, part2)]
fn parse_input_single_race(input: &str) -> Race {
    let lines: Vec<&str> = input.trim().lines().collect();

    // Ensure only digits are concatenated
    let time_str = lines[0]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();
    let distance_str = lines[1]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    let time = time_str.parse::<i64>().unwrap();
    let record_distance = distance_str.parse::<i64>().unwrap();

    Race {
        time,
        record_distance,
    }
}

#[aoc(day6, part1)]
fn part1(races: &[Race]) -> i64 {
    races
        .iter()
        .map(|race| calculate_record_breaks(race))
        .product()
}

fn calculate_record_breaks(race: &Race) -> i64 {
    // Find the first hold_time where hold_time * (time - hold_time) exceeds record_distance
    match (1..race.time).find(|j| j * (race.time - j) > race.record_distance) {
        Some(first_record) => 1 + race.time - (2 * first_record),
        _ => 1,
    }
}

#[aoc(day6, part2)]
fn part2(race: &Race) -> i64 {
    calculate_record_breaks(race)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"Time:      7  15   30\nDistance:  9  40  200"};
    #[test]
    fn test_example1() {
        assert_eq!(part1(&parse_input(SAMPLE)), 288);
    }

    #[test]
    fn test_example2() {
        assert_eq!(part2(&parse_input_single_race(SAMPLE)), 71503);
    }
}
