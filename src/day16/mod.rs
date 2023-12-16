use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr, cmp::max};

use crate::utils::AocError::*;

pub type Coords = (isize, isize);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Hash)]
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
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
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
    visited: &HashSet<(Coords, Direction)>,
) -> HashSet<Coords> {
    use Direction::*;

    let debug = false;

    let mut path = HashSet::new();
    if visited.contains(&(beam.pos, beam.dir.clone())) {
        if debug {
            println!();
            println!("Already visited: {:?}", beam);
            println!();
        }
        return path;
    }
    let mut passed = visited.clone();
    passed.insert((beam.pos, beam.dir.clone()));

    path.insert(beam.pos);

    let mut current = beam.clone();
    if debug {
        println!();
        println!("tracing {:?}", beam);
    }
    loop {
        let tile = map[current.pos.1 as usize][current.pos.0 as usize];
        if debug {
            println!("{:?}, {}", current.pos, tile);
            println!("size {}", path.len());
        }

        // -|/\
        match (tile, current.dir.clone()) {
            ('.', _) => {
                path.insert(current.pos);
            }
            ('-', d) if d == Left || d == Right => {
                path.insert(current.pos);
            }
            ('-', d) if d == Up || d == Down => {
                if debug {
                    println!();
                    println!("split -");
                }
                let left_beam = Beam::new(current.pos, Left);
                let left = trace(map, &left_beam, &passed);
                let right_beam = Beam::new(current.pos, Right);
                let right = trace(map, &right_beam, &passed);

                path.extend(left);
                path.extend(right);

                return path;
            }
            ('|', d) if d == Up || d == Down => {
                path.insert(current.pos);
            }
            ('|', d) if d == Left || d == Right => {
                if debug {
                    println!();
                    println!("split |");
                }
                let up_beam = Beam::new(current.pos, Up);
                let up = trace(map, &up_beam, &passed);
                let down_beam = Beam::new(current.pos, Down);
                let down = trace(map, &down_beam, &passed);

                path.extend(up);
                path.extend(down);

                return path;
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
            return path;
        }

        if passed.contains(&(current.pos, current.dir.clone())) {
            return path;
        }
        passed.insert((current.pos, current.dir.clone()));
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> Result<usize> {
    input.iter().for_each(|l| {
        println!("{}", l.iter().join(""));
    });

    let start = Beam::new((0, 0), Direction::Right);
    let visited = trace(input, &start, &HashSet::new());

    let width = input[0].len();
    let height = input.len();
    let mut visual = vec![vec!['.'; width]; height];
    visited.iter().for_each(|p| {
        visual[p.1 as usize][p.0 as usize] = '#';
    });

    visual.iter().for_each(|l| {
        println!("{}", l.iter().join(""));
    });

    Ok(visited.len())
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> Result<usize> {
    let width = input[0].len() as isize;
    let height = input.len() as isize;
    let passed = HashSet::new();

    let max_x = (0..width).map(|x| {
        let start_top = Beam::new((x, 0), Direction::Down);
        let top = trace(input, &start_top, &passed).len();
        let start_bottom = Beam::new((x, height - 1), Direction::Up);
        let bottom = trace(input, &start_bottom, &passed).len();

        max(top, bottom)
    }).max().ok_or(GenericError).context("Could not find max item in x dir")?;

    let max_y = (1..height-1).map(|y| {
        let start_left = Beam::new((0, y), Direction::Right);
        let left = trace(input, &start_left, &passed).len();
        let start_right = Beam::new((width - 1, y), Direction::Left);
        let right = trace(input, &start_right, &passed).len();

        max(left, right)
    }).max().ok_or(GenericError).context("Could not find max item in y dir")?;

    Ok(max(max_x, max_y))
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
