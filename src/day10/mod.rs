use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::utils::AocError::*;

type Coords = (usize, usize);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<HashMap<Coords, char>> {
//     let input = "FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
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

pub fn find_first(map: &HashMap<Coords, char>, start: &Coords) -> Vec<Coords> {
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

pub fn find_next(map: &HashMap<Coords, char>, pos: &Coords) -> Vec<Coords> {
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

pub fn path(map: &HashMap<Coords, char>) -> Option<Vec<Coords>> {
    let start = map.iter().find(|e| *e.1 == 'S')?.0;
    let candidates = find_first(map, start);
    let mut pos = *candidates.first()?;
    let mut previous = *start;
    let mut path = vec![*start, pos];

    while pos != *start {
        let candidates = find_next(map, &pos);
        if candidates.is_empty() {
            return Some(path);
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

    Some(path)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &HashMap<Coords, char>) -> Result<usize> {
    let len = path(input)
        .ok_or(GenericError)
        .context("Could not determine path")?
        .len();
    Ok(len / 2)
}

pub fn rebuild_pipeline(map: &HashMap<Coords, char>, path: &[Coords]) -> Vec<Vec<char>> {
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

pub fn is_start_relevant(path: &[Coords]) -> bool {
    match (path.first(), path.get(1), path.get(path.len() - 2)) {
        (Some(s), Some(f), Some(l)) => {
            let s0 = s.0 as isize;
            let s1 = s.1 as isize;
            let f0 = f.0 as isize;
            let f1 = f.1 as isize;
            let l0 = l.0 as isize;
            let l1 = l.1 as isize;

            println!("({}, {}) -> ({}, {})", f0, f1, l0, l1);

            // |
            let is_vertical_pipe = f0 == l0 && ((l1 > f1 && l1 - f1 == 2) || (l1 < f1 && f1 - l1 == 2));
            // J
            // let is_j_pipe = (s0 < e0 && e0 - s0 == 1 && e1 - s1 == 1) || (s0 > e0 && s1 - e1 == 1 && s0 - e0 == 1);
            let is_j_pipe = (f0 < l0 && l1 < f1 && l0 - f0 == 1 && f1 - l1 == 1 && (s0, s1) == (f0 + 1, f1)) ||
                (f0 > l0 && l1 > f1 && f0 - l0 == 1 && l1 - f1 == 1 && (s0, s1) == (l0 + 1, l1));
            // L
            // let is_l_pipe = (s0 < e0 && e0 - s0 == 1 && e1 - s1 == 1) || (s0 > e0 && s1 - e1 == 1 && s0 - e0 == 1);
            let is_l_pipe = (f0 < l0 && l1 > f1 && l0 - f0 == 1 && l1 - f1 == 1 && (s0, s1) == (f0, f1 + 1)) ||
                (f0 > l0 && f1 > l1 && f0 - l0 == 1 && f1 - l1 == 1 && (s0, s1) == (l0, l1 + 1));

            println!("start: {} or {} or {}", is_vertical_pipe, is_j_pipe, is_l_pipe);


            is_vertical_pipe || is_j_pipe || is_l_pipe
        },
        _ => false,
    }
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &HashMap<Coords, char>) -> Result<usize> {
    let path = path(input)
        .ok_or(GenericError)
        .context("Could not determine path")?;

    println!("path {:?}", path);

    let map = rebuild_pipeline(input, &path);

    // whether the S is in here needs to be determined by looking at the input, not hard coding it
    // all examples need it to be left out, the real input requires it to be left in
    // let relevant_pipes = ['J', '|', 'L', 'S'];
    let mut relevant_pipes = vec!['|', 'J', 'L'];
    if is_start_relevant(&path) {
        println!("Start is relevant");
        relevant_pipes.push('S');
    }
    let mut is_inside = false;
    let mut counter = 0;
    map.iter().for_each(|l| {
        l.iter().for_each(|c| {
            if relevant_pipes.contains(c) {
                is_inside = !is_inside;
            }

            if *c == '.' && is_inside {
                counter += 1;
            }
        });
    });

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

    fn sample3() -> &'static str {
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
    }

    fn sample4() -> &'static str {
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
    }

    fn sample5() -> &'static str {
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
    }

    #[test]
    fn part2_sample3() -> Result<()> {
        let data = input(sample3())?;
        Ok(assert_eq!(4, solve_part2(&data)?))
    }

    #[test]
    fn part2_sample4() -> Result<()> {
        let data = input(sample4())?;
        Ok(assert_eq!(8, solve_part2(&data)?))
    }

    #[test]
    fn part2_sample5() -> Result<()> {
        let data = input(sample5())?;
        Ok(assert_eq!(10, solve_part2(&data)?))
    }
}
