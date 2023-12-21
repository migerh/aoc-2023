use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use crate::utils::AocError::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Tile {
    Plot,
    Start,
    Wall,
}

impl Tile {
    fn from_char(s: char) -> Result<Self> {
        Ok(match s {
            '#' => Self::Wall,
            '.' => Self::Plot,
            'S' => Self::Start,
            _ => Err(GenericError).context("Error parsing Tile")?,
        })
    }
}

type Coords = (isize, isize);
type Map = HashMap<Coords, Tile>;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Result<Map> {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    input
        .lines()
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, v)| Ok(((x as isize, y as isize), Tile::from_char(v)?)))
        })
        .collect::<Result<HashMap<Coords, Tile>>>()
}

fn successors(map: &Map, pos: &Coords) -> Vec<Coords> {
    [
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
    ]
    .into_iter()
    .filter(|c| map.contains_key(c))
    .filter_map(|c| Some((c, map.get(&c)?)))
    .filter(|(c, t)| **t == Tile::Plot || **t == Tile::Start)
    .map(|(c, _)| c)
    .collect_vec()
}

fn can_reach(map: &Map, pos: &Vec<Coords>) -> Vec<Coords> {
    let mut result = HashSet::new();

    for p in pos {
        for n in successors(map, p) {
            result.insert(n);
        }
    }

    result.into_iter().collect_vec()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    let goal = 64;
    let start = input
        .iter()
        .find(|(_, v)| **v == Tile::Start)
        .map(|(k, _)| *k)
        .context("Could not find start")?;

    let mut result = vec![start];
    for i in 0..goal {
        result = can_reach(input, &result);
    }

    Ok(result.len())
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Map) -> Result<u32> {
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
