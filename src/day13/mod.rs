use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{cmp::max, collections::HashSet, str::FromStr};

use crate::utils::AocError::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
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

    fn compare(
        data: &[Vec<char>],
        except: Option<usize>,
        extract_op: &dyn Fn(&[Vec<char>], usize) -> Vec<char>,
        len_op: &dyn Fn(&[Vec<char>]) -> usize,
        debug: bool,
    ) -> Option<usize> {
        let len = len_op(data);
        let mut findings = vec![];

        for i in 0..(len - 1) {
            let mut found = true;

            for j in 0..(i + 1) {
                let a_index = match i.checked_sub(j) {
                    Some(v) => v,
                    // None => continue,
                    None => {
                        found = false;
                        break;
                    }
                };
                let a = &extract_op(data, a_index);

                let b_index = i + j + 1;
                if b_index >= len {
                    continue;
                }

                let b = &extract_op(data, b_index);

                if debug && i == 7 {
                    println!("compare a: {:?}", a.iter().join(""));
                    println!("compare b: {:?}", b.iter().join(""));
                    println!();
                }

                if !Self::match_lines(a, b) {
                    if debug && i == 7 {
                        println!("no match {} {:?}", a_index, a.iter().join(""));
                        println!("no match {} {:?}", b_index, b.iter().join(""));
                    }
                    found = false;
                    break;
                }
            }

            if debug && i == 7 {
                println!("match? {:?}", found);
            }

            if found {
                findings.push(i);
            }
        }

        if debug {
            println!("{:?}, {:?}", findings, except);
        }

        let foo = findings
            .into_iter()
            .find(|f| if let Some(e) = except { *f != e } else { true });

        println!("{:?}", foo);

        foo
    }

    fn find_mirror(&self, except: Option<Mirror>) -> Option<Mirror> {
        let horizontal_except = match except {
            Some(Mirror::Horizontal(v)) => Some(v),
            _ => None,
        };
        if let Some(v) = Self::compare(
            &self.data,
            horizontal_except,
            &Self::extract_horizontal,
            &Self::size_horizontal,
            false,
        ) {
            let result = Mirror::Horizontal(v + 1);

            if let Some(e) = except.clone() {
                if e != result {
                    println!("except {:?}, result {:?}", except, result);
                    return Some(result);
                }
            } else {
                println!("except {:?}, result {:?}", except, result);
                return Some(result);
            }
        }

        let vertical_except = match except {
            Some(Mirror::Vertical(v)) => Some(v),
            _ => None,
        };
        Self::compare(
            &self.data,
            vertical_except,
            &Self::extract_vertical,
            &Self::size_vertical,
            true,
        )
        .map(|x| Mirror::Vertical(x + 1))
    }

    fn print(&self) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                print!("{}", self.data[i][j]);
            }
            println!();
        }
    }

    fn smudge(&self) -> Option<Mirror> {
        let original = self.find_mirror(None)?;
        println!("original {:?}", original);
        let mut candidates = HashSet::new();

        for i in 0..self.data.len() {
            // if i != 6 {
            //     continue;
            // }
            for j in 0..self.data[i].len() {
                // if j != 10 {
                //     continue;
                // }
                println!("patching {} {}", i, j);
                let mut patched = self.data.clone();
                patched[i][j] = match self.data[i][j] {
                    '.' => '#',
                    '#' => '.',
                    _ => panic!("No way"),
                };
                let smudged = Self { data: patched };
                smudged.print();

                if let Some(p) = smudged.find_mirror2(Some(original.clone())) {
                    if p != original {
                        candidates.insert(p.clone());
                    }
                }
            }
        }

        // println!("{:?}", candidates);
        // if candidates.is_empty() {
        //     self.print();
        // }

        candidates.into_iter().find(|v| original != *v)
    }

    // start fresh

    fn to_num(line: &[char]) -> usize {
        (0..line.len())
            .map(|i| if line[i] == '#' { 1 << i } else { 0 })
            .sum()
    }

    fn compare2(data: &[usize]) -> Vec<usize> {
        (0..data.len()-1)
            .filter(|i| {
                // i = 3:
                //    3 - 4, 2 - 5, 1 - 6
                //
                // i = 0:
                //    0 - 1
                //
                // i = 5:
                //    5 - 6, 4 - 7, 3 - 8, 2 - 9, 1 - 10, 0 - 11
                let mut found = true;

                for j in 0..i + 1 {
                    let a_index = match i.checked_sub(j) {
                        Some(v) => v,
                        None => {
                            found = false;
                            break;
                        }
                    };
                    let a = data[a_index];

                    let b_index = (i + 1) + j;
                    if b_index >= data.len() {
                        continue;
                    }
                    let b = data[b_index];

                    if a != b {
                        found = false;
                        break;
                    }
                }

                found
            })
            .map(|x| x + 1)
            .collect_vec()
    }

    fn find_mirror2(&self, ignore: Option<Mirror>) -> Option<Mirror> {
        let rows = self.data.iter().map(|l| Self::to_num(l)).collect_vec();
        let cols = (0..Self::size_vertical(&self.data))
            .map(|i| Self::extract_vertical(&self.data, i))
            .map(|l| Self::to_num(&l))
            .collect_vec();

        Self::compare2(&cols)
            .into_iter()
            .map(Mirror::Vertical)
            .chain(
                Self::compare2(&rows)
                    .into_iter()
                    .map(Mirror::Horizontal),
            )
            .find(|m| {
                if let Some(ignore) = &ignore {
                    m != ignore
                } else {
                    true
                }
            })

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
    let mirrors = input
        .iter()
        .map(|m| m.find_mirror2(None))
        .collect::<Option<Vec<_>>>()
        .ok_or(GenericError)
        .context("One or more did not have mirrors")?;
    let result = mirrors
        .iter()
        .map(|m| match m {
            Mirror::Horizontal(v) => *v * 100,
            Mirror::Vertical(v) => *v,
        })
        .sum();

    Ok(result)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Map]) -> Result<usize> {
    let mirrors = input
        .iter()
        // .skip(1)
        // .take(1)
        .filter_map(|m| match m.smudge() {
            Some(v) => Some(v),
            None => Some(Mirror::Horizontal(0)),
        })
        .collect::<Vec<_>>();
    let debug = mirrors
        .iter()
        .enumerate()
        .map(|(i, m)| match m {
            Mirror::Horizontal(v) => *v * 100,
            Mirror::Vertical(v) => *v,
        })
        .collect_vec();
    println!("{:?}", debug);

    let result = mirrors
        .iter()
        .map(|m| match m {
            Mirror::Horizontal(v) => *v * 100,
            Mirror::Vertical(v) => *v,
        })
        .sum();

    Ok(result)
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
