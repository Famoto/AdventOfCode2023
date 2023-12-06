use aoc_runner_derive::{aoc, aoc_generator};

// Define a structure to hold the race details
struct Race {
    time: i64,
    record_distance: i64,
}

// Parse the input into a vector of Race structs
#[aoc_generator(day6, part1)]
fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let times: Vec<i64> = lines[0]
        .split_whitespace()
        .skip(1) // Skip the "Time:" part
        .map(|num| num.parse().unwrap())
        .collect();

    let distances: Vec<i64> = lines[1]
        .split_whitespace()
        .skip(1) // Skip the "Distance:" part
        .map(|num| num.parse().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(races: &[Race]) -> i64 {
    races
        .iter()
        .map(|race| {
            (0..race.time)
                .filter(|&hold_time| {
                    let travel_time = race.time - hold_time;
                    let speed = hold_time;
                    let distance = speed * travel_time;
                    distance > race.record_distance
                })
                .count() as i64
        })
        .product()
}
#[aoc_generator(day6, part2)]
fn parse_input_single_race(input: &str) -> Race {
    let lines: Vec<&str> = input.trim().lines().collect();

    // Ensure only digits are concatenated
    let time_str = lines[0]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();
    let distance_str = lines[1]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

    let time = time_str.parse::<i64>().unwrap();
    let record_distance = distance_str.parse::<i64>().unwrap();

    Race {
        time,
        record_distance,
    }
}

#[aoc(day6, part2)]
fn part2(race: &Race) -> i64 {
    (0..race.time)
        .filter(|&hold_time| {
            let travel_time = race.time - hold_time;
            let speed = hold_time;
            let distance = speed * travel_time;
            distance > race.record_distance
        })
        .count() as i64
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
