use aoc_runner_derive::{aoc, aoc_generator};
use num_integer::lcm;

mod utils {
    pub struct Network {
        pub steps: Vec<usize>,
        pub map: std::collections::HashMap<String, [String; 2]>,
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> utils::Network {
    use itertools::Itertools;

    let mut lines = input.lines();
    let steps = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid input"),
        })
        .collect();
    let mut map = std::collections::HashMap::new();
    for line in lines.skip(1) {
        let (key, steps) = line.split('=').collect_tuple().unwrap();
        let key = key.trim().to_string();
        let (left, right) = steps
            .split(',')
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .collect_tuple()
            .unwrap();
        map.insert(key, [left, right]);
    }

    utils::Network { steps, map }
}

#[aoc(day8, part1)]
fn part1(network: &utils::Network) -> u64 {
    let mut current_node = "AAA".to_string();
    let mut count = 0;

    for step in network.steps.iter().cycle() {
        count += 1;
        current_node = network.map[&current_node][*step].clone();
        if current_node == "ZZZ" {
            break;
        }
    }

    count
}

#[aoc(day8, part2)]
fn part2(network: &utils::Network) -> u64 {
    use part2_utils::Path;

    let mut paths: Vec<Path> = network
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| Path::new(k))
        .collect();

    let mut step_counter = 0;
    let mut paths_in_progress: Vec<usize> = (0..paths.len()).collect();

    while !paths_in_progress.is_empty() {
        for &path_index in &paths_in_progress.clone() {
            let path = &mut paths[path_index];
            path.current_node = network.map[&path.current_node]
                [network.steps[step_counter % network.steps.len()]]
            .clone();
            if path.current_node.ends_with('Z') {
                path.steps_taken = (step_counter + 1) as u64;
                paths_in_progress.retain(|&i| i != path_index);
            }
        }
        step_counter += 1;
    }

    paths.iter().fold(0, |acc, path| {
        if acc == 0 {
            path.steps_taken
        } else {
            lcm(acc, path.steps_taken)
        }
    })
}

mod part2_utils {
    pub struct Path {
        pub current_node: String,
        pub steps_taken: u64,
    }

    impl Path {
        pub fn new(node: &String) -> Self {
            Self {
                current_node: node.clone(),
                steps_taken: 0,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(part1(&parse(SAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        const SAMPLE: &str = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(part2(&parse(SAMPLE)), 6);
    }
}
