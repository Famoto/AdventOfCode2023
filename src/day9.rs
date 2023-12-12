use aoc_runner_derive::{aoc, aoc_generator};

// Day 9: Advent of Code Challenge

// Generator function to parse input data
#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

// Part 1: Calculate a specific sum based on input data
#[aoc(day9, part1)]
fn solve_part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|seq| predict_next_recursive(seq)).sum()
}

// Part 2: Modification of input data and calculation
#[aoc(day9, part2)]
fn solve_part2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|arr| {
            let mut reversed = arr.clone();
            reversed.reverse();
            predict_next_recursive(&reversed)
        })
        .sum()
}

// Utility function to recursively predict the next value
fn predict_next_recursive(row: &[i32]) -> i32 {
    let next_row: Vec<i32> = row.windows(2).map(|w| w[1] - w[0]).collect();
    row.last().unwrap_or(&0)
        + next_row
            .first()
            .map_or(0, |_| predict_next_recursive(&next_row))
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&parse_input(SAMPLE_INPUT)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&parse_input(SAMPLE_INPUT)), 2);
    }
}
