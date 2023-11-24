use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Thing {
    items: Vec<u32>,
}

impl FromStr for Thing {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .lines()
            .map(|l| Ok(l.parse::<u32>()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(Thing { items })
    }
}

#[aoc_generator(dayXX)]
pub fn input_generator(input: &str) -> Result<Vec<Thing>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Thing::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(dayXX, part1)]
pub fn solve_part1(input: &[Thing]) -> Result<u32> {
    let max = input
        .iter()
        .map(|elf| elf.items.iter().sum::<u32>())
        .max()
        .ok_or(GenericError)
        .context("Error while finding the elf with the most food")?;
    Ok(max)
}

#[aoc(dayXX, part2)]
pub fn solve_part2(input: &[Thing]) -> Result<u32> {
    let top_three = input
        .iter()
        .map(|elf| elf.items.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum();

    Ok(top_three)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Thing>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
