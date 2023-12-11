use anyhow::Result;
use itertools::Itertools;
use std::cmp::{max, min};

use crate::utils::AocError::*;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect_vec())
}

type Coords = (usize, usize);

fn find_empty(m: &[Vec<char>]) -> Option<(Vec<usize>, Vec<usize>)> {
    let empty_rows = m
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            if l.iter().all(|c| *c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec();

    let width = m.get(0)?.len();
    let height = m.len();
    let empty_cols = (0..width)
        .filter(|i| {
            (0..height).map(|j| m[j][*i]).all(|c| c == '.')
        })
        .collect_vec();

    Some((empty_rows, empty_cols))
}

fn find_galaxies(m: &[Vec<char>]) -> Vec<Coords> {
    m.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.clone()
                .into_iter()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((y, x)) } else { None })
        })
        .collect_vec()
}

fn special_distance(
    empty: (Vec<usize>, Vec<usize>),
    from: &Coords,
    to: &Coords,
    age: usize,
) -> usize {
    let start_0 = min(to.0, from.0);
    let start_1 = min(to.1, from.1);
    let end_0 = max(to.0, from.0);
    let end_1 = max(to.1, from.1);

    let mut distance = (end_0 - start_0, end_1 - start_1);

    // account for expansion
    distance.0 += empty
        .0
        .iter()
        .filter(|e| (start_0..=end_0).contains(e))
        .count()
        * (age - 1);
    distance.1 += empty
        .1
        .iter()
        .filter(|e| (start_1..=end_1).contains(e))
        .count()
        * (age - 1);

    distance.0 + distance.1
}

fn solve(m: &[Vec<char>], age: usize) -> Result<usize> {
    let galaxies = find_galaxies(m);
    let empty = find_empty(m).ok_or(GenericError)?;

    let sum = galaxies
        .iter()
        .combinations(2)
        .map(|pair| special_distance(empty.clone(), pair[0], pair[1], age))
        .sum::<usize>();

    Ok(sum)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    solve(input, 2)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    fn input() -> Result<Vec<Vec<char>>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(374, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(82000210, solve_part2(&data)?))
    }
}
