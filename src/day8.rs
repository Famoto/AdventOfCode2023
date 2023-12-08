use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

// Node structure
type Node = (String, String);
// Map for the network
type Network = HashMap<String, Node>;

#[aoc_generator(day8, part1)]
fn parse1(input: &str) -> (Network, Vec<char>) {
    let mut network = Network::new();
    let mut instructions = Vec::new();
    let mut first_line = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if first_line {
            instructions.extend(line.chars());
            first_line = false;
        } else {
            let parts: Vec<&str> = line.split(" = ").collect();
            let key = parts[0].to_string();
            let values: Vec<&str> = parts[1][1..parts[1].len() - 1].split(", ").collect();
            network.insert(key, (values[0].to_string(), values[1].to_string()));
        }
    }

    (network, instructions)
}

#[aoc(day8, part1)]
fn part1(input: &(Network, Vec<char>)) -> u64 {
    let (network, instructions) = input;
    let mut current_node = "AAA".to_string();
    let mut step_count = 0u64;

    let mut instruction_pointer = 0;
    while current_node != "ZZZ" {
        let (left, right) = &network[&current_node];
        current_node = if instructions[instruction_pointer] == 'L' {
            left
        } else {
            right
        }
        .to_string();

        step_count += 1;
        instruction_pointer = (instruction_pointer + 1) % instructions.len();
    }

    step_count
}
#[aoc_generator(day8, part2)]
fn parse2(input: &str) -> (Network, Vec<char>) {
    let (network, instructions) = parse1(input); // Reusing the parse1 function
    (network, instructions)
}

#[aoc(day8, part2)]
fn part2(input: &(Network, Vec<char>)) -> u64 {
    let (network, instructions) = input;
    let mut current_nodes: Vec<String> = network
        .keys()
        .filter(|&k| k.ends_with('A'))
        .cloned()
        .collect();
    let mut step_count = 0u64;

    while !current_nodes.iter().all(|node| node.ends_with('Z')) {
        let mut next_nodes = Vec::new();

        for node in &current_nodes {
            let (left, right) = &network[node];
            // Convert step_count to usize for indexing
            let instruction_index = (step_count as usize) % instructions.len();
            let next_node = if instructions[instruction_index] == 'L' {
                left
            } else {
                right
            };
            next_nodes.push(next_node.clone());
        }

        current_nodes = next_nodes;
        step_count += 1;
    }

    step_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE1: &str = indoc! {"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"};

    const SAMPLE2: &str = indoc! {"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"};

    #[test]
    fn part1_example() {
        let parsed_input = parse1(SAMPLE1);
        assert_eq!(part1(&parsed_input), 2);
    }
    #[test]
    fn part2_example() {
        let parsed_input = parse2(SAMPLE2);
        assert_eq!(part2(&parsed_input), 6);
    }
}
