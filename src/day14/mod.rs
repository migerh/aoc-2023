use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        use Tile::*;

        Ok(match c {
            'O' => RoundRock,
            '#' => CubeRock,
            '.' => Empty,
            _ => Err(GenericError).context("Unknown tile type")?,
        })
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<Tile>>> {
//     let input = "O....#....
// O.OO#....#
// .....##...
// OO.#O....O
// .O.....O#.
// O.#..O.#.#
// ..O..#O..O
// .......O..
// #....###..
// #OO..#....";
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| Ok(Tile::from_char(c)?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

fn tilt_north(dish: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut dish = dish.to_vec();

    loop {
        let mut has_moved = false;

        for i in 1..dish.len() {
            for j in 0..dish[i].len() {
                let c = dish[i][j];
                let above = dish[i - 1][j];

                if c == Tile::RoundRock && above == Tile::Empty {
                    dish[i - 1][j] = Tile::RoundRock;
                    dish[i][j] = Tile::Empty;
                    has_moved = true;
                }
            }
        }

        if !has_moved {
            break;
        }
    }

    dish
}

fn eval(dish: &[Vec<Tile>]) -> usize {
    let len = dish.len();
    dish.iter()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|r| **r == Tile::RoundRock).count() * (len - i))
        .sum()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Vec<Tile>]) -> Result<usize> {
    let tilted = tilt_north(input);
    let result = eval(&tilted);
    Ok(result)
}

fn cycle(dish: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut dish = tilt_north(dish);

    // west
    loop {
        let mut has_moved = false;

        for i in 0..dish.len() {
            for j in 1..dish[i].len() {
                let c = dish[i][j];
                let left = dish[i][j - 1];

                if c == Tile::RoundRock && left == Tile::Empty {
                    dish[i][j - 1] = Tile::RoundRock;
                    dish[i][j] = Tile::Empty;
                    has_moved = true;
                }
            }
        }

        if !has_moved {
            break;
        }
    }

    // south
    loop {
        let mut has_moved = false;

        for i in (0..dish.len() - 1).rev() {
            for j in 0..dish[i].len() {
                let c = dish[i][j];
                let below = dish[i + 1][j];

                if c == Tile::RoundRock && below == Tile::Empty {
                    dish[i + 1][j] = Tile::RoundRock;
                    dish[i][j] = Tile::Empty;
                    has_moved = true;
                }
            }
        }

        if !has_moved {
            break;
        }
    }

    // east
    loop {
        let mut has_moved = false;

        for i in 0..dish.len() {
            for j in (0..dish[i].len() - 1).rev() {
                let c = dish[i][j];
                let right = dish[i][j + 1];

                if c == Tile::RoundRock && right == Tile::Empty {
                    dish[i][j + 1] = Tile::RoundRock;
                    dish[i][j] = Tile::Empty;
                    has_moved = true;
                }
            }
        }

        if !has_moved {
            break;
        }
    }

    dish
}

fn print(dish: &[Vec<Tile>]) {
    dish.iter().for_each(|l| {
        let l = l
            .iter()
            .map(|r| match r {
                Tile::RoundRock => 'O',
                Tile::CubeRock => '#',
                Tile::Empty => '.',
            })
            .join("");
        println!("{}", l)
    });
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Vec<Tile>]) -> Result<usize> {
    // let reps = 1_000_000_000;
    let reps = 200;
    let mut evals = vec![];
    (0..reps).fold(input.to_vec(), |acc, _| {
        evals.push(eval(&acc));
        cycle(&acc)
    });

    // determined manually
    let offset = 122;
    let cycle_size = 18;

    // let offset = 3;
    // let cycle_size = 7;

    let rest = (1_000_000_000 - offset) % cycle_size;
    let result = *evals.iter().skip(offset).skip(rest).next().ok_or(GenericError).context("Could not find result")?;


    // let mut offset = 0;
    // let mut first = 0;
    // // increase offset
    // for off in 0..200 {
    //     let candidate = evals[off];
    //     let others = evals.iter().filter(|e| **e == candidate).count();
    //     if others > 0 {
    //         first = candidate;
    //         offset = off;
    //     }
    // }

    // let cycle_size = evals.iter().skip(offset + 1).find_position(|e| first == **e).ok_or(GenericError).context("Could not find cycle")?.0;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Vec<Tile>>> {
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
