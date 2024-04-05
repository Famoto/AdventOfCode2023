use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day24)]
fn parse(input: &str) -> utils::Hail {
    use itertools::Itertools;
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once('@').unwrap();
            let position = position
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            let velocity = velocity
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            utils::Hailstone { position, velocity }
        })
        .collect_vec()
        .into()
}

#[aoc(day24, part1)]
fn part1(input: &utils::Hail) -> usize {
    input.count_intersections((200_000_000_000_000, 400_000_000_000_000))
}

mod utils {
    use itertools::Itertools;

    #[derive(derive_more::Deref)]
    pub struct Hail(Vec<Hailstone>);

    impl From<Vec<Hailstone>> for Hail {
        fn from(hailstones: Vec<Hailstone>) -> Self {
            Self(hailstones)
        }
    }

    impl Hail {
        pub fn count_intersections(&self, workspace: (i128, i128)) -> usize {
            let lines = self.iter().map(Hailstone::path_as_line).collect_vec();
            (0..self.len())
                .flat_map(|i| (i + 1..self.len()).map(move |j| (i, j)))
                .filter(|&(i, j)| {
                    lines[i].intersect(&lines[j]).is_some_and(|intersect| {
                        (intersect.0 - self[i].position.0).signum() == (self[i].velocity.0).signum()
                            && (intersect.0 - self[j].position.0).signum()
                                == (self[j].velocity.0).signum()
                            && (intersect.1 - self[i].position.1).signum()
                                == (self[i].velocity.1).signum()
                            && (intersect.1 - self[j].position.1).signum()
                                == (self[j].velocity.1).signum()
                            && intersect.0 >= workspace.0
                            && intersect.0 <= workspace.1
                            && intersect.1 >= workspace.0
                            && intersect.1 <= workspace.1
                    })
                })
                .count()
        }
    }

    pub struct Hailstone {
        pub position: (i128, i128, i128),
        pub velocity: (i128, i128, i128),
    }

    impl Hailstone {
        pub fn path_as_line(&self) -> Line {
            Line {
                a: self.velocity.1,
                py1: -self.velocity.0,
                c: self.velocity.0 * self.position.1 - self.velocity.1 * self.position.0,
            }
        }
    }

    pub struct Line {
        a: i128,
        py1: i128,
        c: i128,
    }

    impl Line {
        pub fn intersect(&self, other: &Self) -> Option<(i128, i128)> {
            if (self.a * other.py1 - other.a * self.py1) == 0 {
                None
            } else {
                Some((
                    (self.py1 * other.c - other.py1 * self.c)
                        / (self.a * other.py1 - other.a * self.py1),
                    (self.c * other.a - other.c * self.a)
                        / (self.a * other.py1 - other.a * self.py1),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn part1_example() {
        assert_eq!(parse(SAMPLE).count_intersections((7, 27)), 2);
    }
}
