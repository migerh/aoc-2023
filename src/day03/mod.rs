use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{str::FromStr, cmp::{max, min}, collections::HashMap};

use crate::utils::AocError::*;

type Map = Vec<Vec<char>>;

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Result<Map> {
//     let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &Map) -> Result<u32> {
    let mut visual = input.clone();
    let mut relevant_parts = vec![];

    for y in 0..input.len() {
        let mut buf = vec![];
        let mut start = 0;
        let mut in_number = false;
        for x in 0..input[y].len() {
            if !input[y][x].is_ascii_digit() || x == input[y].len() - 1 {
                if !in_number {
                    continue;
                }

                if input[y][x].is_ascii_digit() && x == input[y].len() - 1 {
                    buf.push(input[y][x]);
                }

                let num = buf.iter().collect::<String>().parse::<u32>()?;
                println!("candidate {}, start {}, end {}, y {}", num, start, x, y);
                let mut has_symbol = false;
                println!("y from {} to {}", max(1, y) - 1, min(input.len() - 1, y + 1));
                for ys in (max(1, y) - 1)..=min(input.len() - 1, y + 1) {
                    println!("y {}", ys);
                    for xs in (max(1, start) - 1)..=min(input[y].len() - 1, x) {
                        println!("x {}", xs);
                        println!("symbol? {}", input[ys][xs]);
                        if !input[ys][xs].is_ascii_digit() && input[ys][xs] != '.' {
                            has_symbol = true;
                            break;
                        }
                    }
                    if has_symbol {
                        break;
                    }
                }

                buf = vec![];
                in_number = false;
                if has_symbol {
                    relevant_parts.push(num);
                } else {
                    for i in start..x {
                        visual[y][i] = 'X';
                    }
                }
            } else {
                if !in_number {
                    start = x;
                }
                in_number = true;
                buf.push(input[y][x]);
            }
        }
    }

    for y in 0..visual.len() {
        for x in 0..visual[y].len() {
            print!("{}", visual[y][x]);
        }
        println!();
    }

    println!("{:?}", relevant_parts);
    Ok(relevant_parts.iter().sum())
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &Map) -> Result<u32> {
    let mut visual = input.clone();
    let mut map = HashMap::new();
    let mut relevant_parts = vec![];

    for y in 0..input.len() {
        let mut buf = vec![];
        let mut start = 0;
        let mut in_number = false;
        for x in 0..input[y].len() {
            if !input[y][x].is_ascii_digit() || x == input[y].len() - 1 {
                if !in_number {
                    continue;
                }

                if input[y][x].is_ascii_digit() && x == input[y].len() - 1 {
                    buf.push(input[y][x]);
                }

                let num = buf.iter().collect::<String>().parse::<u32>()?;
                println!("candidate {}, start {}, end {}, y {}", num, start, x, y);
                let mut has_symbol = false;
                println!("y from {} to {}", max(1, y) - 1, min(input.len() - 1, y + 1));
                for ys in (max(1, y) - 1)..=min(input.len() - 1, y + 1) {
                    println!("y {}", ys);
                    for xs in (max(1, start) - 1)..=min(input[y].len() - 1, x) {
                        println!("x {}", xs);
                        println!("symbol? {}", input[ys][xs]);
                        if input[ys][xs] == '*' {
                            map.entry((ys, xs)).and_modify(|v: &mut Vec<u32>| v.push(num)).or_insert(vec![num]);
                        }
                    }
                }

                buf = vec![];
                in_number = false;
                if has_symbol {
                    relevant_parts.push(num);
                } else {
                    for i in start..x {
                        visual[y][i] = 'X';
                    }
                }
            } else {
                if !in_number {
                    start = x;
                }
                in_number = true;
                buf.push(input[y][x]);
            }
        }
    }

    let result = map.iter().filter_map(|(k, v)| if v.len() == 2 { Some(v[0] * v[1]) } else { None }).sum::<u32>();

    println!("{:?}", map);
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // fn sample() -> &'static str {
    //     ""
    // }

    // fn input() -> Result<Vec<Thing>> {
    //     input_generator(sample())
    // }

    // #[test]
    // fn part1_sample() -> Result<()> {
    //     let data = input()?;
    //     Ok(assert_eq!(0, solve_part1(&data)?))
    // }

    // #[test]
    // fn part2_sample() -> Result<()> {
    //     let data = input()?;
    //     Ok(assert_eq!(0, solve_part2(&data)?))
    // }
}
