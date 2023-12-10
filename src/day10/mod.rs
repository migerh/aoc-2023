use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use crate::utils::AocError::*;

type Coords = (usize, usize);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<HashMap<Coords, char>> {
//    let input = ".....
//.S-7.
//.|.|.
//.L-J.
//.....";
//
//    let input = "..F7.
//.FJ|.
//SJ.L7
//|F--J
//LJ...";

    let mut map = HashMap::new();
    input
        .lines()
        .filter(|s| !s.is_empty())
        .enumerate()
        .for_each(|(i, l)| {
            l.chars().enumerate().for_each(|(j, c)| {
                map.entry((j, i)).or_insert(c);
            });
        });

    Ok(map)
}

pub fn find_next(map: &HashMap<Coords, char>, start: &Coords) -> Vec<Coords> {
    let mut pos = start.clone();
    let mut next = vec![];
    if let Some(f) = map.get(&(start.0, start.1 - 1)) {
        if *f == '|' || *f == 'F' || *f == '7' {
            next.push((start.0, start.1 - 1));
        }
    }

    if let Some(f) = map.get(&(start.0 + 1, start.1)) {
        if *f == '-' || *f == 'J' || *f == '7' {
            next.push((start.0 + 1, start.1));
        }
    }

    if let Some(f) = map.get(&(start.0, start.1 + 1)) {
        if *f == '|' || *f == 'L' || *f == 'J' {
            next.push((start.0, start.1 + 1));
        }
    }

    if let Some(f) = map.get(&(start.0 - 1, start.1)) {
        if *f == '-' || *f == 'L' || *f == 'F' {
            next.push((start.0 - 1, start.1));
        }
    }

    next
}

pub fn find_next2(map: &HashMap<Coords, char>, pos: &Coords) -> Vec<Coords> {
    if let Some(p) = map.get(pos) {
        match p {
            '|' => vec![(pos.0, pos.1-1), (pos.0, pos.1+1)],
            '-' => vec![(pos.0+1, pos.1), (pos.0-1, pos.1)],
            'L' => vec![(pos.0+1, pos.1), (pos.0, pos.1-1)],
            'J' => vec![(pos.0-1, pos.1), (pos.0, pos.1-1)],
            '7' => vec![(pos.0-1, pos.1), (pos.0, pos.1+1)],
            'F' => vec![(pos.0+1, pos.1), (pos.0, pos.1+1)],
            _ => vec![],
        }
    } else {
        vec![]
    }
}

pub fn length(map: &HashMap<Coords, char>) -> Option<usize> {
    let start = map.iter().find(|e| *e.1 == 'S')?.0;
    println!("{:?}", start);
    let candidates = find_next(map, start);
    println!("{:?}", candidates);
    let mut pos = *candidates.first()?;
    let mut previous = *start;
    let mut counter = 1;

    println!("{:?}, {:?}", start, pos);

    while pos != *start {
        counter += 1;
        let candidates = find_next2(map, &pos);
        if candidates.is_empty() {
            println!("No candidates found");
            return Some(counter);
        }
        println!("pos {:?}, cands {:?}, counter {}", pos, candidates, counter);
        if let Some(next) = candidates.into_iter().find(|s| *s != previous) {
            previous = pos;
            pos = next;
        } else {
            break;
        }
    }

    Some(counter)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &HashMap<Coords, char>) -> Result<usize> {
    let len = length(input)
        .ok_or(GenericError)
        .context("Could not traverse")?;
    Ok(len / 2)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &HashMap<Coords, char>) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Thing>> {
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
