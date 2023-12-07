mod hand;

use std::str::FromStr;

use anyhow::{Result, Context};

use hand::Hand;

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Vec<Hand>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Hand::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &[Hand]) -> Result<usize> {
    let mut hands = input.to_vec();
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum::<usize>();

    Ok(result)
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &[Hand]) -> Result<usize> {
    let mut hands = input.iter().map(|h| h.to_joker()).collect::<Vec<_>>();
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum::<usize>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }

    fn input() -> Result<Vec<Hand>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(6440, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(5905, solve_part2(&data)?))
    }
}
