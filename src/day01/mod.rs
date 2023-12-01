use anyhow::{Context, Result};

use crate::utils::AocError::*;

fn normalise_digit(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        v => v,
    }
}

fn from_str(s: &str, patterns: &[&str]) -> Result<u32> {
    let first_p = patterns
        .iter()
        .enumerate()
        .filter_map(|(i, n)| s.find(n).map(|p| (i, p)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .ok_or(GenericError)
        .context("Could not determine min value, it seems there are no numbers")?;
    let first = normalise_digit(patterns[first_p.0]);

    let s_rev = s.chars().rev().collect::<String>();
    let last_p = patterns
        .iter()
        .map(|n| n.chars().rev().collect::<String>())
        .enumerate()
        .filter_map(|(i, n)| s_rev.find(&n).map(|p| (i, p)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .ok_or(GenericError)
        .context("Could not determine min value, it seems there are no numbers")?;
    let last = normalise_digit(patterns[last_p.0]);

    let value = format!("{}{}", first, last).parse::<u32>()?;
    Ok(value)
}

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Result<Vec<String>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.to_owned())
        .collect::<Vec<_>>())
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &[String]) -> Result<u32> {
    let patterns = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let max = input
        .iter()
        .map(|l| from_str(l, &patterns))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<u32>();
    Ok(max)
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &[String]) -> Result<u32> {
    let patterns = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let max = input
        .iter()
        .map(|l| from_str(l, &patterns))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<u32>();

    Ok(max)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "two13nine
eight1wo3three
abcone26threexyz
xtwone37four
4nineeightseven2
zoneight234
7pqrst9sixteen"
    }

    fn input() -> Result<Vec<String>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(234, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(281, solve_part2(&data)?))
    }
}
