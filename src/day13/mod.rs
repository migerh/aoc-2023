use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug)]
pub struct Map {
    data: Vec<Vec<char>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect::<Vec<_>>();
        Ok(Map { data })
    }
}

impl Map {
    fn extract_vertical(data: &[Vec<char>], index: usize) -> Vec<char> {
        data.iter().cloned().map(|l| l[index]).collect_vec()
    }

    fn size_vertical(data: &[Vec<char>]) -> usize {
        data[0].len()
    }

    fn smudge(&self) -> Option<Mirror> {
        let original = self.find_mirror(None)?;
        let mut candidates = HashSet::new();

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                let mut patched = self.data.clone();
                patched[i][j] = match self.data[i][j] {
                    '.' => '#',
                    '#' => '.',
                    _ => panic!("No way"),
                };

                let smudged = Self { data: patched };
                if let Some(p) = smudged.find_mirror(Some(original.clone())) {
                    if p != original {
                        candidates.insert(p.clone());
                    }
                }
            }
        }

        candidates.into_iter().find(|v| original != *v)
    }

    fn to_num(line: &[char]) -> usize {
        (0..line.len())
            .map(|i| if line[i] == '#' { 1 << i } else { 0 })
            .sum()
    }

    fn compare(data: &[usize]) -> Vec<usize> {
        (0..data.len() - 1)
            .filter(|i| {
                let mut found = true;

                for j in 0..i + 1 {
                    let a_index = match i.checked_sub(j) {
                        Some(v) => v,
                        None => {
                            found = false;
                            break;
                        }
                    };
                    let a = data[a_index];

                    let b_index = (i + 1) + j;
                    if b_index >= data.len() {
                        continue;
                    }
                    let b = data[b_index];

                    if a != b {
                        found = false;
                        break;
                    }
                }

                found
            })
            .map(|x| x + 1)
            .collect_vec()
    }

    fn find_mirror(&self, ignore: Option<Mirror>) -> Option<Mirror> {
        let rows = self.data.iter().map(|l| Self::to_num(l)).collect_vec();
        let cols = (0..Self::size_vertical(&self.data))
            .map(|i| Self::extract_vertical(&self.data, i))
            .map(|l| Self::to_num(&l))
            .collect_vec();

        Self::compare(&cols)
            .into_iter()
            .map(Mirror::Vertical)
            .chain(Self::compare(&rows).into_iter().map(Mirror::Horizontal))
            .find(|m| {
                if let Some(ignore) = &ignore {
                    m != ignore
                } else {
                    true
                }
            })
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<Map>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Map::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Map]) -> Result<usize> {
    Ok(input
        .iter()
        .filter_map(|m| m.find_mirror(None))
        .map(|m| match m {
            Mirror::Horizontal(v) => v * 100,
            Mirror::Vertical(v) => v,
        })
        .sum())
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Map]) -> Result<usize> {
    Ok(input
        .iter()
        .filter_map(|m| m.smudge())
        .map(|m| match m {
            Mirror::Horizontal(v) => v * 100,
            Mirror::Vertical(v) => v,
        })
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    fn input() -> Result<Vec<Map>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(405, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(400, solve_part2(&data)?))
    }
}
