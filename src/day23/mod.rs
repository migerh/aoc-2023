use anyhow::{Context, Error, Result};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, str::FromStr};

use crate::utils::AocError::*;

type Coords = (usize, usize);

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
//     let input = "#.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#";

//     let input = "#.######
// #......#
// #.####.#
// #....#.#
// ####.#.#
// ###..#.#
// ###.##.#
// ###....#
// ######.#";
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
    visited: &[Vec<bool>],
    delta: &(isize, isize),
    ignore_slopes: bool,
) -> Option<Coords> {
    let width = width as isize;
    let height = height as isize;

    let p = (p.0 as isize + delta.0, p.1 as isize + delta.1);

    if p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height {
        let p = (p.0 as usize, p.1 as usize);

        if visited[p.1][p.0] {
            return None;
        }

        if ignore_slopes && map[p.1][p.0] != '#' {
            return Some(p);
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
    visited: &Vec<Vec<bool>>,
    width: usize,
    height: usize,
    p: &Coords,
    ignore_slopes: bool,
) -> Vec<(Coords, usize)> {
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut p = *p;
    let mut count = 1;

    // loop {
        let next = directions
            .iter()
            .filter_map(|d| check_candidate(map, width, height, &p, visited, d, ignore_slopes))
            .collect_vec();
    
        // if next.len() != 1 {
             return next.into_iter().map(|n| (n, count)).collect_vec();
        // }

        // if next.is_empty() {
        //     return vec![];
        // }

        // if next.len() == 1 {
        //     count += 1;
        //     p = next[0];
        // }
    // }
}

fn find_longest_path(
    map: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    p: &Coords,
    end: &Coords,
    len: usize,
    ignore_slopes: bool,
) -> Option<usize> {
    if p == end {
        return Some(len);
    }

    let next = successors(map, visited, width, height, p, ignore_slopes);

    next.into_iter()
        .map(|(n, l)| {
            visited[n.1][n.0] = true;
            let result = find_longest_path(map, visited, width, height, &n, end, len + 1, ignore_slopes);
            visited[n.1][n.0] = false;
            result
        })
        .max()?
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    let width = input[0].len();
    let height = input.len();

    let start = find_start(input).context("Could not find start")?;
    let end = find_end(input).context("Could not find end")?;
    let mut visited = vec![vec![false; width]; height];
    visited[start.1][start.0] = true;

    let result = find_longest_path(input, &mut visited, width, height, &start, &end, 0, false)
        .context("Could not find longest path")?;

    Ok(result)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    let width = input[0].len();
    let height = input.len();

    let start = find_start(input).context("Could not find start")?;
    let end = find_end(input).context("Could not find end")?;
    let mut visited = vec![vec![false; width]; height];
    visited[start.1][start.0] = true;

    let result = find_longest_path(input, &mut visited, width, height, &start, &end, 0, true)
        .context("Could not find longest path")?;

    Ok(result)
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
