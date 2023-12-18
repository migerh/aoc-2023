use anyhow::{Context, Error, Result};
use itertools::{Itertools, MinMaxResult};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use crate::utils::AocError::*;

type Coords = (isize, isize);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub enum Tile {
    Inside,
    Outside,
    Border,
    #[default]
    Unknown,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => Err(GenericError).context("Could not parse direction")?,
        })
    }

    pub fn from_digit(d: u32) -> Result<Self> {
        Ok(match d {
            3 => Self::Up,
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            _ => Err(GenericError).context("Could not parse direction")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    dir: Direction,
    len: u32,
    color: String,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^(?P<dir>\w) (?P<len>\d+?) \((?P<color>.+?)\)$").unwrap();
        }

        let (dir, len, color) = RE
            .captures(s)
            .and_then(|cap| {
                let dir = cap.name("dir").map(|v| v.as_str())?.to_string();
                let len = cap.name("len").map(|v| v.as_str())?.to_string();
                let color = cap.name("color").map(|v| v.as_str())?.to_string();

                Some((dir, len, color))
            })
            .context("Error during parse")?;

        let dir = Direction::from_char(
            dir.chars()
                .next()
                .ok_or(GenericError)
                .context("No dir found")?,
        )?;
        let len = len.parse::<u32>()?;

        Ok(Instruction { dir, len, color })
    }
}

impl Instruction {
    pub fn part2(&self) -> Result<Self> {
        let len = self.color.chars().skip(1).take(5).collect::<String>();
        let len = u32::from_str_radix(&len, 16)?;

        let dir = self
            .color
            .chars()
            .nth(6)
            .ok_or(GenericError)
            .context("Could not parse direction (part 2)")?
            .to_string()
            .parse::<u32>()?;
        let dir = Direction::from_digit(dir)?;
        let color = "".to_string();

        Ok(Self { dir, len, color })
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

pub fn walk(
    map: &mut HashMap<Coords, Vec<String>>,
    start: &Coords,
    dir: &Coords,
    len: u32,
    color: &String,
) -> Coords {
    (1..=len).for_each(|c| {
        let c = c as isize;
        let p = (start.0 + c * dir.0, start.1 + c * dir.1);
        map.entry(p)
            .and_modify(|v| v.push(color.clone()))
            .or_insert(vec![color.clone()]);
    });

    let len = len as isize;
    (start.0 + len * dir.0, start.1 + len * dir.1)
}

pub fn trace(instr: &[Instruction], start: Coords) -> Result<HashMap<Coords, Vec<String>>> {
    use Direction::*;
    let mut result = HashMap::new();
    let mut pos = start;

    instr.iter().for_each(|i| {
        pos = match i.dir {
            Up => walk(&mut result, &pos, &(0, -1), i.len, &i.color),
            Right => walk(&mut result, &pos, &(1, 0), i.len, &i.color),
            Down => walk(&mut result, &pos, &(0, 1), i.len, &i.color),
            Left => walk(&mut result, &pos, &(-1, 0), i.len, &i.color),
        }
    });

    Ok(result)
}

pub fn to_grid(map: &HashMap<Coords, Vec<String>>) -> Result<Vec<Vec<Tile>>> {
    let minmax_x = match map.iter().map(|(k, _)| k.0).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => Err(GenericError).context("min max x")?,
    };
    let minmax_y = match map.iter().map(|(k, _)| k.1).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => Err(GenericError).context("min max y")?,
    };

    let width = minmax_x.1 - minmax_x.0 + 1 + 2;
    let height = minmax_y.1 - minmax_y.0 + 1 + 2;
    if width <= 0 && height <= 0 {
        return Err(GenericError).context("Invalid size");
    }

    let width = width as usize;
    let height = height as usize;
    let offset_x = minmax_x.0 - 1;
    let offset_y = minmax_y.0 - 1;
    let mut grid = vec![vec![Tile::Unknown; width]; height];

    map.iter().for_each(|(k, _)| {
        let x = (k.0 - offset_x) as usize;
        let y = (k.1 - offset_y) as usize;
        grid[y][x] = Tile::Border;
    });

    Ok(grid)
}

pub fn neighbors(
    grid: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    pos: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if pos.0 > 0 {
        result.push((pos.0 - 1, pos.1));
    }

    if pos.0 < width - 1 {
        result.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        result.push((pos.0, pos.1 - 1));
    }

    if pos.1 < height - 1 {
        result.push((pos.0, pos.1 + 1));
    }

    result
}

pub fn fill_outside(grid: &mut Vec<Vec<Tile>>) -> Option<()> {
    let height = grid.len();
    let width = grid.get(0)?.len();

    let start = (0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        let tile = grid[p.1][p.0].clone();
        if tile == Tile::Outside || tile == Tile::Border {
            continue;
        }

        grid[p.1][p.0] = Tile::Outside;
        let next = neighbors(grid, width, height, p);
        for n in next {
            queue.push_back(n);
        }
    }

    Some(())
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Instruction]) -> Result<usize> {
    let map = trace(input, (0, 0))?;
    let mut grid = to_grid(&map)?;

    fill_outside(&mut grid)
        .ok_or(GenericError)
        .context("Could not fill")?;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!(
                "{}",
                match grid[y][x] {
                    Tile::Border => '#',
                    Tile::Outside => ' ',
                    Tile::Inside => '#',
                    Tile::Unknown => '.',
                }
            );
        }
        println!();
    }

    let height = grid.len();
    let width = grid
        .get(0)
        .ok_or(GenericError)
        .context("grid is empty")?
        .len();
    let number_of_outside = grid
        .iter()
        .map(|r| r.iter().filter(|t| **t == Tile::Outside).count())
        .sum::<usize>();

    Ok(height * width - number_of_outside)
}

pub fn size(map: &HashMap<Coords, Vec<String>>) -> Result<(isize, isize, usize, usize)> {
    let minmax_x = match map.iter().map(|(k, _)| k.0).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => Err(GenericError).context("min max x")?,
    };
    let minmax_y = match map.iter().map(|(k, _)| k.1).minmax() {
        MinMaxResult::MinMax(x, y) => (x, y),
        _ => Err(GenericError).context("min max y")?,
    };

    let width = minmax_x.1 - minmax_x.0 + 1 + 2;
    let height = minmax_y.1 - minmax_y.0 + 1 + 2;

    Ok((
        minmax_x.0 - 1,
        minmax_y.0 - 1,
        width as usize,
        height as usize,
    ))
}
pub fn neighbors_map(
    offset_x: isize,
    offset_y: isize,
    width: usize,
    height: usize,
    pos: Coords,
) -> Vec<Coords> {
    let width = width as isize;
    let height = height as isize;

    let mut result = vec![];

    if pos.0 > offset_x {
        result.push((pos.0 - 1, pos.1));
    }

    if pos.0 < width - 1 {
        result.push((pos.0 + 1, pos.1));
    }

    if pos.1 > offset_y {
        result.push((pos.0, pos.1 - 1));
    }

    if pos.1 < height - 1 {
        result.push((pos.0, pos.1 + 1));
    }

    result
}

pub fn fill_map_outside(
    map: &mut HashMap<Coords, Tile>,
    offset_x: isize,
    offset_y: isize,
    width: usize,
    height: usize,
) -> Option<()> {
    let start = (offset_x, offset_y);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        let tile = map.get(&p).unwrap_or(&Tile::Unknown).clone();
        if tile == Tile::Outside || tile == Tile::Border {
            continue;
        }

        map.entry(p)
            .and_modify(|t| *t = Tile::Outside)
            .or_insert(Tile::Outside);
        let next = neighbors_map(offset_x, offset_y, width, height, p);
        for n in next {
            queue.push_back(n);
        }
    }

    Some(())
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Instruction]) -> Result<usize> {
    let input = input
        .iter()
        .map(|i| i.part2())
        .collect::<Result<Vec<_>>>()?;

    let map = trace(&input, (0, 0))?;
    let (offset_x, offset_y, width, height) = size(&map)?;
    println!("{}, {} -> {}, {}", offset_x, offset_y, width, height);
    let mut map = map
        .keys()
        .map(|k| (*k, Tile::Border))
        .collect::<HashMap<Coords, Tile>>();

    fill_map_outside(&mut map, offset_x, offset_y, width, height).ok_or(GenericError).context("Could not fill map")?;
    let number_outside = map.iter().filter(|(_, v)| **v == Tile::Outside).count();

    Ok(height * width - number_outside)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Instruction>> {
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
