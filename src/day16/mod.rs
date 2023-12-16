use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::{cmp::max, collections::HashSet};

use crate::utils::AocError::*;

type Set<T> = HashSet<T>;
pub type Coords = (isize, isize);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub struct Beam {
    pos: Coords,
    dir: Direction,
}

impl Beam {
    pub fn new(pos: Coords, dir: Direction) -> Self {
        Self { pos, dir }
    }

    pub fn next_pos(&self) -> Coords {
        use Direction::*;

        match self.dir {
            Up => (self.pos.0, self.pos.1 - 1),
            Right => (self.pos.0 + 1, self.pos.1),
            Down => (self.pos.0, self.pos.1 + 1),
            Left => (self.pos.0 - 1, self.pos.1),
        }
    }

    pub fn next(&self) -> Self {
        Beam::new(self.next_pos(), self.dir.clone())
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect::<Vec<_>>())
}

pub fn is_within(map: &[Vec<char>], pos: &Coords) -> bool {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

pub fn trace(
    map: &[Vec<char>],
    beam: &Beam,
    path: &mut Set<Coords>,
    visited: &mut Set<(Coords, Direction)>,
) {
    use Direction::*;

    if visited.contains(&(beam.pos, beam.dir.clone())) {
        return;
    }
    visited.insert((beam.pos, beam.dir.clone()));

    let mut current = beam.clone();
    loop {
        let tile = map[current.pos.1 as usize][current.pos.0 as usize];

        match (tile, current.dir.clone()) {
            ('.', _) => {
                path.insert(current.pos);
            }
            ('-', d) if d == Left || d == Right => {
                path.insert(current.pos);
            }
            ('-', d) if d == Up || d == Down => {
                let left_beam = Beam::new(current.pos, Left);
                trace(map, &left_beam, path, visited);
                let right_beam = Beam::new(current.pos, Right);
                trace(map, &right_beam, path, visited);

                return;
            }
            ('|', d) if d == Up || d == Down => {
                path.insert(current.pos);
            }
            ('|', d) if d == Left || d == Right => {
                let up_beam = Beam::new(current.pos, Up);
                trace(map, &up_beam, path, visited);
                let down_beam = Beam::new(current.pos, Down);
                trace(map, &down_beam, path, visited);

                return;
            }
            ('\\', d) => {
                path.insert(current.pos);
                current.dir = if d == Up {
                    Left
                } else if d == Right {
                    Down
                } else if d == Down {
                    Right
                } else {
                    Up
                };
            }
            ('/', d) => {
                path.insert(current.pos);
                current.dir = if d == Up {
                    Right
                } else if d == Right {
                    Up
                } else if d == Down {
                    Left
                } else {
                    Down
                }
            }
            _ => panic!("No way"),
        }

        current = current.next();
        if !is_within(map, &current.pos) {
            return;
        }

        if visited.contains(&(current.pos, current.dir.clone())) {
            return;
        }
        visited.insert((current.pos, current.dir.clone()));
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    let start = Beam::new((0, 0), Direction::Right);
    let mut path = Set::new();
    let mut visited = Set::new();
    trace(input, &start, &mut path, &mut visited);

    // Ok(visited.iter().map(|(c, _)| c).collect::<Set<_>>().len())

    Ok(path.len())
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    let width = input[0].len() as isize;
    let height = input.len() as isize;

    let max_x = (0..width).into_par_iter()
        .map(|x| {
            let start_top = Beam::new((x, 0), Direction::Down);
            let mut path_top = Set::new();
            let mut visited_top = Set::new();
            trace(input, &start_top, &mut path_top, &mut visited_top);
            let top = path_top.len();

            let start_bottom = Beam::new((x, height - 1), Direction::Up);
            let mut path_bottom = Set::new();
            let mut visited_bottom = Set::new();
            trace(input, &start_bottom, &mut path_bottom, &mut visited_bottom);
            let bottom = path_bottom.len();

            max(top, bottom)
        })
        .max()
        .ok_or(GenericError)
        .context("Could not find max item in x dir")?;

    let max_y = (1..height - 1).into_par_iter()
        .map(|y| {
            let start_left = Beam::new((0, y), Direction::Right);
            let mut path_left = Set::new();
            let mut visited_left = Set::new();
            trace(input, &start_left, &mut path_left, &mut visited_left);
            let left = path_left.len();

            let start_right = Beam::new((width - 1, y), Direction::Left);
            let mut path_right = Set::new();
            let mut visited_right = Set::new();
            trace(input, &start_right, &mut path_right, &mut visited_right);
            let right = path_right.len();

            max(left, right)
        })
        .max()
        .ok_or(GenericError)
        .context("Could not find max item in y dir")?;

    Ok(max(max_x, max_y))
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
    }

    fn input() -> Result<Vec<Vec<char>>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(46, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(51, solve_part2(&data)?))
    }
}
