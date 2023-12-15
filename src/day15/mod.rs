use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug, Clone)]
pub struct Lens {
    label: String,
    focal: u32,
}

impl FromStr for Lens {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split('=');
        let label = split
            .next()
            .ok_or(GenericError)
            .context("No label")?
            .to_string();
        let focal = split
            .next()
            .ok_or(GenericError)
            .context("No focal")?
            .parse::<u32>()?;

        Ok(Lens { label, focal })
    }
}

#[derive(Debug, Clone)]
pub struct Bin {
    lenses: Vec<Lens>,
}

impl Bin {
    pub fn new() -> Bin {
        Bin { lenses: vec![] }
    }

    pub fn power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, v)| (i + 1) * (v.focal as usize))
            .sum()
    }
}

impl Default for Bin {
    fn default() -> Self {
        Self::new()
    }
}

pub fn hash(s: &str) -> u32 {
    let mut val = 0;
    for c in s.chars() {
        let ascii = c as u8 as u32;
        val += ascii;
        val *= 17;
        val %= 256;
    }

    val
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Vec<String>> {
    Ok(input
        .lines()
        .find(|s| !s.is_empty())
        .ok_or(GenericError)
        .context("Empty input")?
        .split(',')
        .map(|v| v.to_string())
        .collect_vec())
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[String]) -> Result<u32> {
    Ok(input.iter().map(|s| hash(s)).sum())
}

pub fn index(s: &str) -> Result<usize> {
    if s.contains('=') {
        Ok(hash(
            s.split('=')
                .next()
                .ok_or(GenericError)
                .context("No label")?,
        ) as usize)
    } else if s.contains('-') {
        Ok(hash(
            s.split('-')
                .next()
                .ok_or(GenericError)
                .context("No label2")?,
        ) as usize)
    } else {
        Err(GenericError).context("No valid lens")
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[String]) -> Result<usize> {
    let mut boxes = vec![Bin::new(); 256];
    for lens in input {
        let index = index(lens)?;
        let mut b = boxes[index].clone();

        if lens.contains('=') {
            let l = Lens::from_str(lens)?;
            if let Some(pos) = b
                .lenses
                .clone()
                .iter()
                .find_position(|v| v.label == l.label)
            {
                b.lenses[pos.0].focal = l.focal;
            } else {
                b.lenses.push(l);
            }
        } else {
            let label = lens
                .split('-')
                .next()
                .ok_or(GenericError)
                .context("No label")?;
            if let Some(pos) = b.lenses.iter().find_position(|v| v.label == label) {
                b.lenses.remove(pos.0);
            }
        }
        boxes[index] = b;
    }

    let power = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * b.power())
        .sum();

    Ok(power)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    fn input() -> Result<Vec<String>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(1320, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(145, solve_part2(&data)?))
    }
}
