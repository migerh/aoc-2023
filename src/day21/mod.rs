use anyhow::{Context, Result};
use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};

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

fn successors(map: &Map, pos: &Coords, size: &Option<Coords>) -> Vec<Coords> {
    if let Some(s) = size {
        [
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
        ]
        .into_iter()
        .map(|c| (c, (c.0.rem_euclid(s.0), c.1.rem_euclid(s.1))))
        .filter_map(|(p, c)| Some((p, c, map.get(&c)?)))
        .filter(|(_, _, t)| **t == Tile::Plot || **t == Tile::Start)
        .map(|(p, _, _)| p)
        .collect_vec()
    } else {
        [
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
        ]
        .into_iter()
        .filter(|c| map.contains_key(c))
        .filter_map(|c| Some((c, map.get(&c)?)))
        .filter(|(_, t)| **t == Tile::Plot || **t == Tile::Start)
        .map(|(c, _)| c)
        .collect_vec()
    }
}

fn can_reach(map: &Map, pos: &Vec<Coords>, size: &Option<Coords>) -> Vec<Coords> {
    let mut result = HashSet::new();

    for p in pos {
        for n in successors(map, p, size) {
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
    for _ in 0..goal {
        result = can_reach(input, &result, &None);
    }

    Ok(result.len())
}

fn size(map: &Map) -> Result<Coords> {
    let (_, max_x) = match map.iter().map(|(k, _)| k.0).minmax() {
        MinMaxResult::NoElements => Err(GenericError).context("Map has no size")?,
        MinMaxResult::OneElement(e) => (e, e),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    let (_, max_y) = match map.iter().map(|(k, _)| k.1).minmax() {
        MinMaxResult::NoElements => Err(GenericError).context("Map has no size")?,
        MinMaxResult::OneElement(e) => (e, e),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    Ok((max_x + 1, max_y + 1))
}

fn diff_at(steps: usize, start: usize, cycle: usize, diffs: &[usize], offsets: &[usize]) -> usize {
    let observe = (steps - start) - (steps - start) % cycle;
    (observe / cycle) * diffs[(steps - start) % cycle] + offsets[(steps - start) % cycle]
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let goal = 26501365;
    let start = input
        .iter()
        .find(|(_, v)| **v == Tile::Start)
        .map(|(k, _)| *k)
        .context("Could not find start")?;

    let size = Some(size(input)?);

    let mut result = vec![start];
    let mut last = 0;
    let mut diffs = vec![];
    let mut results = vec![];
    for _ in 0..460 {
        result = can_reach(input, &result, &size);
        diffs.push(result.len() - last);
        last = result.len();
        results.push(result.len());
    }

    // these parameters are specific to my personal input and
    // most likely won't work with other people's input.
    let start = 193;
    let cycle = 131;
    let diff = diffs
        .iter()
        .enumerate()
        .skip(start)
        .take(cycle)
        .map(|(i, v)| diffs[i + cycle] - *v)
        .collect_vec();
    let offsets = diffs.iter().skip(start).take(cycle).cloned().collect_vec();

    let mut result = results[start - 1];
    for i in start..goal {
        result += diff_at(i, start, cycle, &diff, &offsets)
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
    }

    fn input() -> Result<Map> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(42, solve_part1(&data)?))
    }

    // skip this one as it takes a few seconds
    // #[test]
    // fn part2_sample() -> Result<()> {
    //     let data = input()?;
    //     Ok(assert_eq!(469538157381253, solve_part2(&data)?))
    // }
}
