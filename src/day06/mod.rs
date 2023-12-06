use anyhow::{Context, Result};

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Race {
    time: u128,
    distance: u128,
}

fn parse_line(s: &str) -> Result<Vec<u128>> {
    let mut split = s.split(':');
    let nums_str = split.nth(1).ok_or(GenericError).context("Could not parse nums")?;
    nums_str.trim().split(' ').filter(|v| !v.is_empty()).map(|v| Ok(v.trim().parse::<u128>()?)).collect::<Result<Vec<_>>>()
}

#[aoc_generator(day06)]
pub fn input_generator(input: &str) -> Result<Vec<Race>> {
    let mut lines = input
        .lines()
        .filter(|s| !s.is_empty());

    let times = lines
        .next()
        .ok_or(GenericError)
        .context("Could not parse times")?;
    let time = parse_line(times)?;

    let distances = lines
        .next()
        .ok_or(GenericError)
        .context("Could not parse distances")?;
    let distance = parse_line(distances)?;

    let races = time.into_iter().zip(distance).map(|(t, d)| Race { time: t, distance: d }).collect::<Vec<_>>();

    Ok(races)
}

fn how_to_win_race(r: &Race) -> u128 {
    let mut win = 0;

    for i in 0..=r.time {
        let speed = i;
        let distance = (r.time - i) * speed;

        if distance > r.distance {
            win += 1;
        }
    }
    
    win
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &[Race]) -> Result<u128> {
    let result = input.iter().map(how_to_win_race).product::<u128>();
    Ok(result)
}

pub fn concat_numbers<I>(iter: I) -> Result<u128> where I: Iterator<Item = u128> {
    Ok(iter.fold("".to_string(), |acc, val| format!("{}{}", acc, val)).parse::<u128>()?)

}

#[aoc(day06, part2)]
pub fn solve_part2(input: &[Race]) -> Result<u128> {
    let time = concat_numbers(input.iter().map(|r| r.time))?;
    let distance = concat_numbers(input.iter().map(|r| r.distance))?;
    let input = vec![Race { time, distance }];

    let result = input.iter().map(how_to_win_race).product::<u128>();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }

    fn input() -> Result<Vec<Race>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(288, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(71503, solve_part2(&data)?))
    }
}
