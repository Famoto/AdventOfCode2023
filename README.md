# Advent of Code

<p>
  <a href="https://github.com/Famoto/AdventOfCode2023/actions/workflows/rust.yml">   <img alt="Rust"   src="https://github.com/Famoto/AdventOfCode2023/actions/workflows/rust.yml/badge.svg"></a>
</p>

My solutions to [Advent of Code](https://adventofcode.com) puzzles.

### [Advent of Code 2023](https://adventofcode.com/2023)

<table>
<tr><th>Day</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

|   D   | Puzzle                                                        |           Code           |
| :---: | ------------------------------------------------------------- |:------------------------:|
|   1   | [Trebuchet?!](https://adventofcode.com/2023/day/1)            | [`day1.rs`](src/day1.rs) |
|   2   | [Cube Conundrum](https://adventofcode.com/2023/day/2)         | [`day2.rs`](src/day2.rs) |
|   3   | [Gear Ratios](https://adventofcode.com/2023/day/3)            | [`day3.rs`](src/day3.rs) |
|   4   | [Scratchcards](https://adventofcode.com/2023/day/4)           | [`day4.rs`](src/day4.rs) |
|   5   | [If You Give A Seed ...](https://adventofcode.com/2023/day/5) | [`day5.rs`](src/day5.rs) |

</td><td>

| Generator  |   Runner   |
|:----------:|:----------:|
| 24.767 µs  | 651.81 µs  |
|  3.426 µs  |  1.233 ms  |
| 19.878 µs  |  2.073 ms  |
|  1.427 ms  | 600.524 µs |
| 182.395 µs | 41.789 µs  |

</td><td>

| Generator |  Runner   |
|:---------:|:---------:|
|  992 ns   | 3.846 ms  |
| 2.244 µs  | 1.233 ms  |
| 8.506 µs  | 1.408 ms  |
| 1.904 ms  | 25.819 s  |
| 19.617 µs |  5.178 s  |

</td></tr>
</table>

## Instructions

<details open>
<summary><h3><a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust</h3></summary>

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/271355/rocket-ship-rocket.svg" width="14" height="14"></a> Get Answers and Run Performance Benchmarks

Thanks to [`cargo-aoc`](https://github.com/gobanos/cargo-aoc), answers and performance benchmarks for all days are obtainable by running the main binary.

```bash
cargo run --release
```

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/269868/lab.svg" width="14" height="14"></a> Test the Correctness of Solutions

All days also include tests using sample inputs from the puzzle descriptions.

```bash
cargo test
```

</details>

### Readme Template from [AndrejOrsula](https://github.com/AndrejOrsula/aoc)
