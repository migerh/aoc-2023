use anyhow::{Context, Error, Result};
use std::{cmp::max, str::FromStr};

use crate::utils::AocError::*;

use Cube::*;

#[derive(Debug)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = Error;

    // Parse '3 blue' or '13 red'
    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.trim().split(' ').map(|s| s.trim());
        let num = split
            .next()
            .map(|s| s.parse::<u32>())
            .ok_or(GenericError)
            .context("Could not parse input, expected number")??;
        let what = split
            .next()
            .ok_or(GenericError)
            .context("Could not parse input, expected color")?;

        Ok(match what {
            "blue" => Blue(num),
            "red" => Red(num),
            "green" => Green(num),
            _ => Err(GenericError).context("Unknown color")?,
        })
    }
}

#[derive(Debug)]
pub struct Draw {
    cubes: Vec<Cube>,
}

impl FromStr for Draw {
    type Err = Error;

    // Parse '3 red, 5 blue, 1 green'
    fn from_str(s: &str) -> Result<Self> {
        let cubes = s
            .split(',')
            .map(Cube::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cubes })
    }
}

impl Draw {
    fn is_valid_for_part1(&self) -> bool {
        self.cubes.iter().all(|e| match e {
            Red(v) => *v <= 12,
            Green(v) => *v <= 13,
            Blue(v) => *v <= 14,
        })
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = Error;

    // Parse a single input line
    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(':');

        let id = split
            .next()
            .ok_or(GenericError)
            .context("Could not read game name")?
            .split(' ')
            .nth(1)
            .map(|v| v.parse::<u32>())
            .ok_or(GenericError)
            .context("Could not read game id")??;

        let draws = split
            .next()
            .ok_or(GenericError)
            .context("Could not parse draws")?
            .split(';')
            .map(Draw::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Game { id, draws })
    }
}

impl Game {
    pub fn is_valid_for_part1(&self) -> bool {
        self.draws.iter().map(|v| v.is_valid_for_part1()).all(|b| b)
    }

    pub fn power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for set in &self.draws {
            for cube in &set.cubes {
                match cube {
                    Red(v) => max_red = max(max_red, *v),
                    Green(v) => max_green = max(max_green, *v),
                    Blue(v) => max_blue = max(max_blue, *v),
                }
            }
        }

        max_red * max_green * max_blue
    }
}

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &[Game]) -> Result<u32> {
    let hash = input
        .iter()
        .map(|g| (g.id, g.is_valid_for_part1()))
        .filter(|(_, v)| *v)
        .map(|(i, _)| i)
        .sum::<u32>();

    Ok(hash)
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &[Game]) -> Result<u32> {
    let result = input.iter().map(|g| g.power()).sum::<u32>();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    fn input() -> Result<Vec<Game>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(8, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(2286, solve_part2(&data)?))
    }
}
