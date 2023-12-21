use anyhow::{Context, Error, Result};
use itertools::{Itertools, MinMaxResult};
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
        .filter(|(c, t)| **t == Tile::Plot || **t == Tile::Start)
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
    for i in 0..goal {
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

fn diff_at(steps: usize, diffs: &Vec<usize>, offsets: &Vec<usize>) -> usize {
    let start = 42;
    let cycle = 11;

    let observe = (steps - start) - (steps - start) % cycle;
    (observe / cycle) * diffs[(steps - start) % cycle] + offsets[(steps - start) % cycle]
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let goal = 5000; //26501365;
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
    for i in 0..150 {
        result = can_reach(input, &result, &size);
        println!("{}: {}, diff: {}", i, result.len(), result.len() - last);
        diffs.push(result.len() - last);
        last = result.len();
        results.push(result.len());
    }

    let start = 42;
    let cycle = 11;
    let diff = diffs
        .iter()
        .enumerate()
        .skip(start)
        .take(cycle)
        .map(|(i, v)| diffs[i + cycle] - *v)
        .collect_vec();
    let offsets = diffs.iter().skip(start).take(cycle).cloned().collect_vec();
    println!("{:?}", diff);

    let mut result = results[start - 1];
    for i in start..goal {
        let d = diff_at(i, &diff, &offsets);
        println!("{}: {}", i, d);
        result += diff_at(i, &diff, &offsets)
    }

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
