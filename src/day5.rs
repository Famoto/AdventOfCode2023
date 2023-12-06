use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use smallvec::SmallVec;
use std::ops::Range;
use rayon::prelude::*;

mod utils {
    pub struct Almanac {
        pub seeds: smallvec::SmallVec<[u32; 20]>,
        pub mappings: smallvec::SmallVec<[Vec<RangeMap>; 7]>,
    }

    pub struct RangeMap {
        pub from: std::ops::Range<u32>,
        pub to_start: u32,
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> utils::Almanac {
    let mut input_sections = input.split("\n\n");

    let seeds = input_sections
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mappings = input_sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let (to_start, start, length) = line
                        .split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .next_tuple()
                        .unwrap();
                    utils::RangeMap {
                        from: Range {
                            start,
                            end: start + length,
                        },
                        to_start,
                    }
                })
                .collect()
        })
        .collect();

    utils::Almanac { seeds, mappings }
}

#[aoc(day5, part1)]
fn part1(almanac: &utils::Almanac) -> u32 {
    almanac.seeds
        .iter()
        .map(|&seed| {
            almanac.mappings.iter().fold(seed, |value, mapping| {
                mapping
                    .iter()
                    .find(|range_map| range_map.from.contains(&value))
                    .map_or(value, |range_map| {
                        value + range_map.to_start - range_map.from.start
                    })
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(almanac: &utils::Almanac) -> u32 {
    almanac.seeds
        .par_chunks(8)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .map(|(start, end)| {
            let seed_range = start..end;
            let mut cache = (
                SmallVec::<[usize; 7]>::from_elem(usize::MAX, almanac.mappings.len()),
                SmallVec::<[i32; 7]>::from_elem(0, almanac.mappings.len()),
            );

            seed_range
                .map(|seed| {
                    almanac.mappings.iter().enumerate().fold(seed, |value, (i, mapping)| {
                        if let Some(range_map) = mapping.get(cache.0[i]) {
                            if range_map.from.contains(&value) {
                                return value.wrapping_add_signed(cache.1[i]);
                            }
                        }
                        mapping
                            .iter()
                            .enumerate()
                            .find(|(_, range_map)| range_map.from.contains(&value))
                            .map_or(value, |(j, range_map)| {
                                cache.0[i] = j;
                                cache.1[i] = range_map.to_start as i32 - range_map.from.start as i32;
                                value.wrapping_add_signed(cache.1[i])
                            })
                    })
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 46);
    }
}