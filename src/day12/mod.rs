use anyhow::{Context, Error, Result};
use itertools::Itertools;
use num::pow;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug)]
pub enum Type {
    Spring(u32),
    Broken(u32),
    Unknown(u32),
}

#[derive(Debug)]
pub struct SpringConfig {
    springs: Vec<char>,
    config: Vec<u32>,
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
            .map(|v| Ok(v.parse::<u32>()?))
            .collect::<Result<Vec<_>>>()?;

        Ok(SpringConfig { springs, config })
    }
}

impl SpringConfig {
    fn partition(springs: &Vec<char>) -> Option<Vec<Type>> {
        let mut current = springs.first()?;
        let mut count = 1;
        let mut partition = vec![];

        springs.iter().skip(1).for_each(|s| {
            if s != current {
                partition.push(match current {
                    '?' => Type::Unknown(count),
                    '.' => Type::Broken(count),
                    '#' => Type::Spring(count),
                    _ => panic!("Should never happen"),
                });
                current = s;
                count = 1;
            } else {
                count += 1;
            }
        });
        partition.push(match current {
            '?' => Type::Unknown(count),
            '.' => Type::Broken(count),
            '#' => Type::Spring(count),
            _ => panic!("Should never happen"),
        });

        Some(partition)
    }

    fn eval(v: &Vec<char>) -> Option<Vec<u32>> {
        let partition = Self::partition(v)?;
        Some(partition.iter().filter_map(|p| match p {
            Type::Spring(v) => Some(*v),
            _ => None
        }).collect_vec())
    }

    pub fn brute_force(&self) -> usize {
        let unknown = self.springs.iter().enumerate().filter_map(|(i, c)| if *c == '?' { Some(i) } else { None }).collect_vec();
        let num_unknown = self.springs.iter().filter(|c| **c == '?').count();
        // println!("unknown: {}, combs: {}, list: {:?}", num_unknown, pow(2, num_unknown), unknown);

        let mut all = vec![];
        for i in 0..pow(2, num_unknown) {
            let mut s = self.springs.clone();
            for j in 0..num_unknown {
                if (1 << j) & i == (1 << j) {
                    s[unknown[j]] = '#';
                } else {
                    s[unknown[j]] = '.';
                }
            }
            all.push(s);
        }

        // println!("{:?}", all);

        let evals = all.iter().filter_map(Self::eval).filter(|v| {
            if v.len() != self.config.len() {
                return false;
            }

            v.iter().enumerate().all(|(i, v)| *v == self.config[i])
        }).collect_vec();

        // println!("{:?}", evals);

        evals.len()
    }

    pub fn combinations(&self) -> Option<u32> {
        Some(12)
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Vec<SpringConfig>> {
//     let input = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(SpringConfig::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[SpringConfig]) -> Result<usize> {
    let result = input
        .iter()
        .map(|s| s.brute_force())
        .sum::<usize>();
    Ok(result)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[SpringConfig]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<SpringConfig>> {
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
