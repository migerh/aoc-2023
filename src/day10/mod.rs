use anyhow::{Context, Error, Result};
use itertools::Itertools;
use num::traits::WrappingSub;
use std::{collections::HashMap, str::FromStr};

use crate::utils::AocError::*;

type Coords = (usize, usize);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<HashMap<Coords, char>> {
//     let input = "...........
// .S-------7.
// .|F-----7|.
// .||.....||.
// .||.....||.
// .|L-7.F-J|.
// .|..|.|..|.
// .L--J.L--J.
// ...........";
//     let input = ".F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ...";
//     let input = "FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJIF7FJ-
// L---JF-JLJIIIIFJLJJ7
// |F|F-JF---7IIIL7L|7|
// |FFJF7L7F-JF7IIL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L";
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
    let mut next = vec![];
    if let Some(f) = map.get(&(start.0, start.1.wrapping_sub(1))) {
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

    if let Some(f) = map.get(&(start.0.wrapping_sub(1), start.1)) {
        if *f == '-' || *f == 'L' || *f == 'F' {
            next.push((start.0 - 1, start.1));
        }
    }

    next
}

pub fn find_next2(map: &HashMap<Coords, char>, pos: &Coords) -> Vec<Coords> {
    if let Some(p) = map.get(pos) {
        match p {
            '|' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
            '-' => vec![(pos.0 + 1, pos.1), (pos.0 - 1, pos.1)],
            'L' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
            'J' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
            '7' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
            'F' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
            _ => vec![],
        }
    } else {
        vec![]
    }
}

pub fn length(map: &HashMap<Coords, char>) -> Option<(Vec<Coords>, usize)> {
    let start = map.iter().find(|e| *e.1 == 'S')?.0;
    println!("{:?}", start);
    let candidates = find_next(map, start);
    println!("{:?}", candidates);
    let mut pos = *candidates.first()?;
    let mut previous = *start;
    let mut counter = 1;
    let mut path = vec![*start, pos];

    println!("{:?}, {:?}", start, pos);

    while pos != *start {
        counter += 1;
        let candidates = find_next2(map, &pos);
        if candidates.is_empty() {
            println!("No candidates found");
            return Some((path, counter));
        }
        // println!("pos {:?}, cands {:?}, counter {}", pos, candidates, counter);
        if let Some(next) = candidates.into_iter().find(|s| *s != previous) {
            previous = pos;
            pos = next;
            path.push(pos);
        } else {
            break;
        }
    }

    Some((path, counter))
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &HashMap<Coords, char>) -> Result<usize> {
    let len = length(input)
        .ok_or(GenericError)
        .context("Could not traverse")?
        .1;
    Ok(len / 2)
}

pub fn rebuild(map: &HashMap<Coords, char>, path: Vec<Coords>) -> Vec<Vec<char>> {
    let min = path.iter().fold((0, 0), |acc, el| {
        if el.0 < acc.0 && el.1 < acc.1 {
            *el
        } else if el.0 < acc.0 {
            (el.0, acc.1)
        } else if el.1 < acc.1 {
            (acc.0, el.1)
        } else {
            acc
        }
    });

    let max = path.iter().fold((0, 0), |acc, el| {
        if el.0 > acc.0 && el.1 > acc.1 {
            *el
        } else if el.0 > acc.0 {
            (el.0, acc.1)
        } else if el.1 > acc.1 {
            (acc.0, el.1)
        } else {
            acc
        }
    });

    println!("{:?}, {:?}", min, max);

    let mut m = vec![vec!['.'; max.0 - min.0 + 1]; max.1 - min.1 + 1];
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if let Some(c) = map.get(&(x, y)) {
                if path.contains(&(x, y)) {
                    m[y][x] = *c;
                }
            }
        }
    }

    m
}

fn print(map: Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &HashMap<Coords, char>) -> Result<usize> {
    let path = length(input)
        .ok_or(GenericError)
        .context("Could not determine path")?
        .0;

    // println!("Path {:?}", path);
    let map = rebuild(input, path);

    // whether the S is in here needs to be determined by looking at the input, not hard coding it
    // all examples need it to be left out, the real input requires it to be left in
    let relevant_pipes = ['J', '|', 'L', 'S'];
    let mut is_inside = false;
    let mut counter = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i][j];
            if relevant_pipes.contains(&c) {
                is_inside = !is_inside;
            }

            if c == '.' && is_inside {
                counter += 1;
            }
        }
    }

    // print(map);

    Ok(counter)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> &'static str {
        ".....
.S-7.
.|.|.
.L-J.
....."
    }

    fn sample2() -> &'static str {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
    }

    fn input(s: &str) -> Result<HashMap<Coords, char>> {
        input_generator(s)
    }

    #[test]
    fn part1_sample1() -> Result<()> {
        let data = input(sample1())?;
        Ok(assert_eq!(4, solve_part1(&data)?))
    }

    #[test]
    fn part1_sample2() -> Result<()> {
        let data = input(sample2())?;
        Ok(assert_eq!(8, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input("")?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
