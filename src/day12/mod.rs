use anyhow::{Context, Error, Result};
use itertools::Itertools;
use memoize::memoize;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct SpringConfig {
    springs: Vec<char>,
    config: Vec<usize>,
}

impl FromStr for SpringConfig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(' ');
        let springs = split
            .next()
            .ok_or(GenericError)
            .context("Spring reading")?
            .chars()
            .collect_vec();
        let config = split
            .next()
            .ok_or(GenericError)
            .context("Config reading")?
            .split(',')
            .map(|v| Ok(v.parse::<usize>()?))
            .collect::<Result<Vec<_>>>()?;

        Ok(SpringConfig { springs, config })
    }
}

impl SpringConfig {
    pub fn unfold(&self) -> Self {
        let springs = vec![
            self.springs.clone(),
            vec!['?'],
            self.springs.clone(),
            vec!['?'],
            self.springs.clone(),
            vec!['?'],
            self.springs.clone(),
            vec!['?'],
            self.springs.clone(),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect_vec();

        let config = [
            self.config.clone(),
            self.config.clone(),
            self.config.clone(),
            self.config.clone(),
            self.config.clone(),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect_vec();

        Self { springs, config }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Vec<SpringConfig>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .map(SpringConfig::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[SpringConfig]) -> Result<usize> {
    Ok(input
        .iter()
        .map(|s| count_possible_solutions(s.springs.clone(), 0, s.config.clone()))
        .sum::<usize>())
}

#[memoize]
fn count_possible_solutions(springs: Vec<char>, matches: usize, config: Vec<usize>) -> usize {
    if springs.is_empty() && matches == 0 && config.is_empty() {
        return 1;
    }

    if springs.is_empty() && config.len() == 1 && matches == config[0] {
        return 1;
    }

    if springs.is_empty() {
        return 0;
    }

    if matches > 0 && config.is_empty() {
        return 0;
    }

    match (springs[0], matches) {
        ('?', 0) => {
            count_possible_solutions(springs[1..].to_vec(), 1, config.clone())
                + count_possible_solutions(springs[1..].to_vec(), 0, config.clone())
        }
        ('?', c) => {
            let mut sub_count =
                count_possible_solutions(springs[1..].to_vec(), c + 1, config.clone());
            if c == config[0] {
                sub_count +=
                    count_possible_solutions(springs[1..].to_vec(), 0, config[1..].to_vec());
            }
            sub_count
        }

        ('#', 0) => count_possible_solutions(springs[1..].to_vec(), 1, config),
        ('#', c) => count_possible_solutions(springs[1..].to_vec(), c + 1, config),

        ('.', 0) => count_possible_solutions(springs[1..].to_vec(), 0, config),
        ('.', x) if x != config[0] => 0,
        ('.', _) => count_possible_solutions(springs[1..].to_vec(), 0, config[1..].to_vec()),

        _ => panic!("No way"),
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[SpringConfig]) -> Result<usize> {
    let result = input
        .iter()
        .map(|s| s.unfold())
        .map(|s| count_possible_solutions(s.springs.clone(), 0, s.config.to_vec()))
        .sum::<usize>();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    fn input() -> Result<Vec<SpringConfig>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(21, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(525152, solve_part2(&data)?))
    }
}
