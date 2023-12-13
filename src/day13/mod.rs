use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{str::FromStr, cmp::max};

use crate::utils::AocError::*;

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
    fn match_lines(a: &[char], b: &[char]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.iter().enumerate().all(|(i, c)| *c == b[i])
    }

    fn extract_horizontal(data: &[Vec<char>], index: usize) -> Vec<char> {
        data[index].to_vec()
    }

    fn size_horizontal(data: &[Vec<char>]) -> usize {
        data.len()
    }

    fn extract_vertical(data: &[Vec<char>], index: usize) -> Vec<char> {
        data.iter().cloned().map(|l| l[index]).collect_vec()
    }

    fn size_vertical(data: &[Vec<char>]) -> usize {
        data[0].len()
    }

    fn compare(data: &[Vec<char>], extract_op: &dyn Fn(&[Vec<char>], usize) -> Vec<char>, len_op: &dyn Fn(&[Vec<char>]) -> usize) -> Option<usize> {
        let len = len_op(data);
        let mut line = None;

        for i in 0..(len-1) {
            let mut found = true;
            println!("looking at line after {}", i);

            for j in 0..(i+1) {
                println!("would compare {} with {}", (i as isize - j as isize), i+j+1);

                let a_index = match i.checked_sub(j) {
                    Some(v) => v,
                    None => continue,
                };
                let a = &extract_op(data, a_index);

                let b_index = i+j+1;
                if b_index >= len {
                    continue;
                }

                println!("comparing {} with {}", a_index, b_index);
                let b = &extract_op(data, b_index);
                if !Self::match_lines(a, b) {
                    found = false;
                    break;
                }
            }
            println!();

            if found {
                println!("found {}", i);
                line = Some(i);
                break;
            }
        }

        line
    }

    fn find_mirror(&self) -> Option<Mirror> {
        if let Some(v) = Self::compare(&self.data, &Self::extract_horizontal, &Self::size_horizontal) {
            return Some(Mirror::Horizontal(v + 1));
        }

        Self::compare(&self.data, &Self::extract_vertical, &Self::size_vertical).map(|x| Mirror::Vertical(x + 1))
    }

    fn smudge(&self) -> Option<usize> {
        None
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<Map>> {
//     let input = "#.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
// 
// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#";
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(Map::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Map]) -> Result<usize> {
    let mirrors = input.iter().map(|m| m.find_mirror()).collect::<Option<Vec<_>>>().ok_or(GenericError).context("One or more did not have mirrors")?;
    let result = mirrors.iter().map(|m| match m {
        Mirror::Horizontal(v) => *v * 100,
        Mirror::Vertical(v) => *v,
    }).sum();

    Ok(result)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Map]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Map>> {
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
