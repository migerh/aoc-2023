use anyhow::{Context, Error, Result};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::{collections::HashSet, str::FromStr};

use crate::utils::AocError::*;

type Coords = (usize, usize);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    let input = "#.######
#......#
#.####.#
#....#.#
####.#.#
###..#.#
###.##.#
###....#
######.#";
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect::<Vec<_>>())
}

fn print(map: &[Vec<char>]) {
    map.iter().for_each(|l| println!("{}", l.iter().join("")));
}

fn find_path(map: &[Vec<char>], line: usize) -> Option<Coords> {
    let pos = map.get(line)?.iter().find_position(|p| **p == '.')?;
    Some((pos.0, line))
}

fn find_start(map: &[Vec<char>]) -> Option<Coords> {
    find_path(map, 0)
}

fn find_end(map: &[Vec<char>]) -> Option<Coords> {
    find_path(map, map.len() - 1)
}

fn check_candidate(
    map: &[Vec<char>],
    width: usize,
    height: usize,
    p: &Coords,
    visited: &[Coords],
    delta: &(isize, isize),
) -> Option<Coords> {
    let width = width as isize;
    let height = height as isize;

    let p = (p.0 as isize + delta.0, p.1 as isize + delta.1);

    if p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height {
        let p = (p.0 as usize, p.1 as usize);

        if visited.contains(&p) {
            return None;
        }

        if map[p.1][p.0] == '.' {
            return Some(p);
        }

        if map[p.1][p.0] == '#' {
            return None;
        }

        return match (map[p.1][p.0], delta) {
            ('>', (1, 0)) => Some(p),
            ('<', (-1, 0)) => Some(p),
            ('^', (0, -1)) => Some(p),
            ('v', (0, 1)) => Some(p),
            _ => None,
        };
    }

    None
}

fn successors(
    map: &[Vec<char>],
    visited: Vec<Coords>,
    width: usize,
    height: usize,
    p: &Coords,
) -> Vec<(Coords, Vec<Coords>)> {
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    directions
        .iter()
        .filter_map(|d| check_candidate(map, width, height, p, &visited, d))
        .map(|p| {
            let mut visited = visited.clone();
            visited.push(p);

            (p, visited)
        })
        .collect_vec()
}

fn find_longest_path(
    map: &[Vec<char>],
    visited: Vec<Coords>,
    width: usize,
    height: usize,
    p: &Coords,
    end: &Coords,
    len: usize,
) -> Option<usize> {
    if p == end {
        return Some(len);
    }

    let next = successors(map, visited, width, height, p);

    next.into_iter()
        .map(|n| find_longest_path(map, n.1.clone(), width, height, &n.0, end, len + 1))
        .max()?
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    print(input);

    let width = input[0].len();
    let height = input.len();

    let start = find_start(input).context("Could not find start")?;
    let end = find_end(input).context("Could not find end")?;
    let visited = vec![start];

    let result = find_longest_path(input, visited, width, height, &start, &end, 0)
        .context("Could not find longest path")?;

    // let result = dijkstra(
    //     &(start, visited.clone()),
    //     |p| successors(input, p.1.clone(), width, height, &p.0),
    //     |p| p.0 == end,
    // )
    // .context("Could not determine longest path")?;

    // let path = result.0.iter().map(|r| r.0).collect_vec();

    println!();
    println!();
    println!();
    // for y in 0..input.len() {
    //     for x in 0..input[y].len() {
    //         if path.contains(&(x, y)) {
    //             print!("O");
    //         } else {
    //             print!("{}", input[y][x]);
    //         }
    //     }
    //     println!();
    // }

    Ok(result)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<u32> {
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
