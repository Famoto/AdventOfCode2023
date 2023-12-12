use aoc_runner_derive::{aoc, aoc_generator};

// Day 11 - Part 1
#[aoc_generator(day11, part1)]
fn parse_galaxies(input: &str) -> Vec<(usize, usize)> {
    utils::parse_galaxies(input)
}

#[aoc(day11, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    utils::calculate_sum_of_distances(input)
}

// Day 11 - Part 2
#[aoc_generator(day11, part2)]
fn parse_expanded_galaxies(input: &str) -> Vec<(usize, usize)> {
    let (expanding_rows, expanding_cols) = utils::find_expanding_dimensions(input);
    let galaxies = utils::find_galaxies(input);
    utils::expand_universe(&galaxies, &expanding_rows, &expanding_cols, 999_999)
}

#[aoc(day11, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    utils::calculate_sum_of_distances(input)
}

mod utils {
    use rayon::prelude::*;
    use std::collections::HashSet;

    #[inline]
    pub(crate) fn parse_galaxies(input: &str) -> Vec<(usize, usize)> {
        let (expanding_rows, expanding_cols) = find_expanding_dimensions(input);
        let galaxies = find_galaxies(input);
        expand_universe(&galaxies, &expanding_rows, &expanding_cols, 1)
    }

    pub fn calculate_sum_of_distances(galaxies: &[(usize, usize)]) -> usize {
        let len = galaxies.len();
        (0..len)
            .into_par_iter()
            .flat_map(move |i| {
                (i + 1..len)
                    .into_par_iter()
                    .map(move |j| manhattan_distance(&galaxies[i], &galaxies[j]))
            })
            .sum()
    }

    #[inline]
    fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        (a.0.abs_diff(b.0)) + (a.1.abs_diff(b.1))
    }

    pub fn find_expanding_dimensions(input: &str) -> (HashSet<usize>, HashSet<usize>) {
        let lines: Vec<&str> = input.lines().collect();
        let row_len = lines.first().map_or(0, |s| s.len());

        (0..row_len)
            .into_par_iter()
            .fold(
                || (HashSet::new(), HashSet::new()),
                |(mut rows, mut cols), i| {
                    let is_row_expanding = lines[i].chars().all(|c| c == '.');
                    let is_col_expanding =
                        lines.iter().all(|line| line.chars().nth(i).unwrap() == '.');

                    if is_row_expanding {
                        rows.insert(i);
                    }
                    if is_col_expanding {
                        cols.insert(i);
                    }

                    (rows, cols)
                },
            )
            .reduce(
                || (HashSet::new(), HashSet::new()),
                |(mut acc_rows, mut acc_cols), (rows, cols)| {
                    acc_rows.extend(rows);
                    acc_cols.extend(cols);
                    (acc_rows, acc_cols)
                },
            )
    }

    pub fn parse_expanded_galaxies(input: &str, expansion_rate: usize) -> Vec<(usize, usize)> {
        // Use find_expanding_dimensions function which returns a tuple of HashSets
        let (expanding_rows, expanding_cols) = find_expanding_dimensions(input);
        let galaxies = find_galaxies(input);

        // Pass the HashSets to expand_universe
        expand_universe(&galaxies, &expanding_rows, &expanding_cols, expansion_rate)
    }

    #[inline]
    pub(crate) fn find_galaxies(input: &str) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                }
            }
        }
        galaxies
    }

    pub fn expand_universe(
        galaxies: &[(usize, usize)],
        expanding_rows: &HashSet<usize>,
        expanding_cols: &HashSet<usize>,
        expansion_rate: usize,
    ) -> Vec<(usize, usize)> {
        let x_limit = galaxies.iter().max_by_key(|(x, _)| x).unwrap().0 + 1;
        let y_limit = galaxies.iter().max_by_key(|(_, y)| y).unwrap().1 + 1;

        // Use TinyVec for efficient vector operations
        let mut expanded_galaxies: Vec<(usize, usize)> = Vec::new();
        let mut n_expanded_rows = 0;

        for y in 0..y_limit {
            if expanding_rows.contains(&y) {
                n_expanded_rows += expansion_rate;
                continue;
            }
            let mut n_expanded_cols = 0;
            for x in 0..x_limit {
                if expanding_cols.contains(&x) {
                    n_expanded_cols += expansion_rate;
                    continue;
                }
                if galaxies.contains(&(x, y)) {
                    expanded_galaxies.push((x + n_expanded_cols, y + n_expanded_rows));
                }
            }
        }
        expanded_galaxies.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_galaxies(SAMPLE)), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&utils::parse_expanded_galaxies(SAMPLE, 9)), 1030);
        assert_eq!(part2(&utils::parse_expanded_galaxies(SAMPLE, 99)), 8410);
    }
}
