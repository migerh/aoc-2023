use anyhow::{Context, Error, Result};
use num::checked_pow;
use std::{cmp::min, str::FromStr};

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let numbers = s
            .split(':')
            .nth(1)
            .ok_or(GenericError)
            .context("Could not find numbers")?
            .trim();
        let mut split = numbers.split('|');

        let winning = parse_numbers(
            split
                .next()
                .ok_or(GenericError)
                .context("Could not parse winners")?
                .trim(),
        )?;

        let numbers = parse_numbers(
            split
                .next()
                .ok_or(GenericError)
                .context("Could not parse winners")?
                .trim(),
        )?;

        Ok(Card { winning, numbers })
    }
}

impl Card {
    fn wins(&self) -> usize {
        self.winning
            .iter()
            .map(|w| if self.numbers.contains(w) { 1 } else { 0 })
            .sum::<usize>()
    }

    fn score(&self) -> Result<u32> {
        let exp = self.wins();
        if exp == 0 {
            Ok(0)
        } else {
            checked_pow(2, exp - 1)
                .ok_or(GenericError)
                .context("Could not pow")
        }
    }
}

fn parse_numbers(s: &str) -> Result<Vec<u32>> {
    s.trim()
        .split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| Ok(v.parse::<u32>()?))
        .collect::<Result<Vec<_>>>()
}

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> Result<Vec<Card>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &[Card]) -> Result<u32> {
    Ok(input.iter().filter_map(|c| c.score().ok()).sum::<u32>())
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &[Card]) -> Result<usize> {
    let mut copies = vec![1_usize; input.len()];

    for (i, c) in input.iter().enumerate() {
        let wins = c.wins();

        let end = min(input.len() - 1, i + wins);
        for j in i + 1..=end {
            copies[j] += copies[i];
        }
    }

    Ok(copies.iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    }

    fn input() -> Result<Vec<Card>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(13, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(30, solve_part2(&data)?))
    }
}
