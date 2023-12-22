use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::utils::AocError::*;

type Coords = (isize, isize, isize);

#[derive(Debug, Clone)]
pub struct Brick {
    from: Coords,
    to: Coords,
}

fn parse_coords(s: &str) -> Result<Coords> {
    let split = s
        .split(',')
        .map(|c| Ok(c.parse::<isize>()?))
        .collect::<Result<Vec<_>>>()?;

    if split.len() < 3 {
        return Err(GenericError).context("Error parsing coords: Not enough components");
    }

    Ok((split[0], split[1], split[2]))
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let split = s.split('~').map(parse_coords).collect::<Result<Vec<_>>>()?;

        if split.len() < 2 {
            return Err(GenericError).context("ERror parsing range: Not enough coords");
        }

        let from = split[0];
        let to = split[1];
        Ok(Brick { from, to })
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Result<Vec<Brick>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Brick::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

impl Brick {
    fn new(from: Coords, to: Coords) -> Self {
        Self { from, to }
    }

    fn is_inside(&self, p: &Coords) -> bool {
        for z in self.from.2..=self.to.2 {
            for y in self.from.1..=self.to.1 {
                for x in self.from.0..=self.to.0 {
                    if (x, y, z) == *p {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn intersect(&self, b: &Brick) -> bool {
        for z in b.from.2..=b.to.2 {
            for y in b.from.1..=b.to.1 {
                for x in b.from.0..=b.to.0 {
                    if self.is_inside(&(x, y, z)) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn fall(&mut self) {
        self.from.2 -= 1;
        self.to.2 -= 1;
    }
}

fn can_fall(i: usize, bricks: &[Brick], ignore: Option<usize>) -> bool {
    let brick = &bricks[i];
    if brick.from.2 == 1 || brick.to.2 == 1 {
        return false;
    }

    let new = Brick::new(
        (brick.from.0, brick.from.1, brick.from.2 - 1),
        (brick.to.0, brick.to.1, brick.to.2 - 1),
    );

    for (idx, b) in bricks.iter().enumerate() {
        if let Some(ignore) = ignore {
            if idx == ignore {
                continue;
            }
        }

        if idx == i {
            continue;
        }

        if b.intersect(&new) {
            return false;
        }
    }

    true
}

fn stabilize(bricks: &[Brick]) -> (Vec<Brick>, usize) {
    let mut bricks = bricks.to_vec();
    let mut has_fallen = vec![false; bricks.len()];

    loop {
        if bricks
            .iter()
            .enumerate()
            .all(|(i, _)| !can_fall(i, &bricks, None))
        {
            break;
        }

        for i in 0..bricks.len() {
            if can_fall(i, &bricks, None) {
                bricks[i].fall();
                has_fallen[i] = true;
            }
        }
    }

    let count = has_fallen.iter().filter(|v| **v).count();
    (bricks, count)
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[Brick]) -> Result<u32> {
    let bricks = stabilize(input).0;

    let len = bricks.len();
    let mut count = 0;

    for i in 0..len {
        if bricks
            .iter()
            .enumerate()
            .all(|(idx, _)| !can_fall(idx, &bricks, Some(i)))
        {
            count += 1;
        }
    }

    Ok(count)
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[Brick]) -> Result<usize> {
    let bricks = stabilize(input).0;

    let len = bricks.len();
    let mut count = 0;

    for i in 0..len {
        let without_i = bricks
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, b)| b)
            .cloned()
            .collect_vec();
        count += stabilize(&without_i).1;
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect1() {
        let b1 = Brick::new((1, 0, 1), (1, 2, 1));
        let b2 = Brick::new((0, 0, 1), (2, 0, 1));

        assert!(b1.intersect(&b2))
    }

    fn sample() -> &'static str {
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    }

    fn input() -> Result<Vec<Brick>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(5, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(7, solve_part2(&data)?))
    }
}
