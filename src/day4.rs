use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(" | ").collect();
            let winning_numbers = parts[0]
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let my_numbers = parts[1]
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (winning_numbers, my_numbers)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(Vec<i64>, Vec<i64>)]) -> i64 {
    input
        .iter()
        .map(|(winning, mine)| {
            let mut points = 0;
            let mut has_match = false;
            for number in mine {
                if winning.contains(number) {
                    if has_match {
                        points *= 2;
                    } else {
                        points = 1;
                        has_match = true;
                    }
                }
            }
            points
        })
        .sum()
}
#[aoc(day4, part2)]
fn part2(input: &[(Vec<i64>, Vec<i64>)]) -> usize {
    let mut total_cards = 0;
    let mut queue: Vec<(usize, usize)> = input.iter().enumerate().map(|(i, _)| (i, 1)).collect();

    while let Some((card_index, copies)) = queue.pop() {
        total_cards += copies;
        let (winning, mine) = &input[card_index];
        let matches = mine.iter().filter(|n| winning.contains(n)).count();

        // Queue up the next set of cards, if any
        for next_card_index in card_index + 1..input.len().min(card_index + 1 + matches) {
            queue.push((next_card_index, copies));
        }
    }

    total_cards
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 13);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 30);
    }
}
