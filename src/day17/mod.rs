use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::utils::AocError::*;

type Coords = (i32, i32);

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u32>>> {
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec())
}

fn turn_left(dir: Coords) -> Coords {
    match dir {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        _ => panic!("No way"),
    }
}

fn turn_right(dir: Coords) -> Coords {
    match dir {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (1, 0),
        _ => panic!("No way"),
    }
}

fn is_inside(pos: Coords, width: usize, height: usize) -> bool {
    pos.0 >= 0 && (pos.0 as usize) < width && pos.1 >= 0 && (pos.1 as usize) < height
}

fn successors(
    map: &[Vec<u32>],
    width: usize,
    height: usize,
    node: &(Coords, Coords, i32),
    ultra: bool,
) -> Vec<((Coords, Coords, i32), u32)> {
    let mut neighbors = vec![];
    let (pos, dir, straight) = node;
    let min_straight = if ultra { 3 } else { 0 };
    let max_straight = if ultra { 10 } else { 3 };

    if *straight < max_straight - 1 {
        let forward = ((pos.0 + dir.0, pos.1 + dir.1), *dir, straight + 1);
        if is_inside(forward.0, width, height) {
            let loss = map[forward.0 .1 as usize][forward.0 .0 as usize];
            neighbors.push((forward, loss));
        }
    }

    if *straight >= min_straight {
        let left_dir = turn_left(*dir);
        let left_pos = (pos.0 + left_dir.0, pos.1 + left_dir.1);
        if is_inside(left_pos, width, height) {
            let loss = map[left_pos.1 as usize][left_pos.0 as usize];
            neighbors.push(((left_pos, left_dir, 0), loss));
        }

        let right_dir = turn_right(*dir);
        let right_pos = (pos.0 + right_dir.0, pos.1 + right_dir.1);
        if is_inside(right_pos, width, height) {
            let loss = map[right_pos.1 as usize][right_pos.0 as usize];
            neighbors.push(((right_pos, right_dir, 0), loss));
        }
    }

    neighbors
}

fn trace(map: &[Vec<u32>], ultra: bool) -> Option<u32> {
    let height = map.len();
    let width = map[0].len();
    let min_straight = if ultra { 3 } else { 0 };

    let start = (0, 0);
    let dir = (1, 0);
    let result = dijkstra(
        &(start, dir, if ultra { -1 } else { 0 }),
        |node| successors(map, width, height, node, ultra),
        |pos| {
            pos.2 >= min_straight
                && pos.0 .0 as usize == width - 1
                && pos.0 .1 as usize == height - 1
        },
    )?;

    Some(result.1)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> Result<u32> {
    Ok(trace(input, false).ok_or(GenericError)?)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> Result<u32> {
    Ok(trace(input, true).ok_or(GenericError)?)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    }

    fn input(s: &str) -> Result<Vec<Vec<u32>>> {
        input_generator(s)
    }

    #[test]
    fn part1_sample1() -> Result<()> {
        let data = input(sample1())?;
        Ok(assert_eq!(102, solve_part1(&data)?))
    }

    fn sample2() -> &'static str {
        "111111111111
999999999991
999999999991
999999999991
999999999991"
    }

    #[test]
    fn part2_sample1() -> Result<()> {
        let data = input(sample1())?;
        Ok(assert_eq!(94, solve_part2(&data)?))
    }

    #[test]
    fn part2_sample2() -> Result<()> {
        let data = input(sample2())?;
        Ok(assert_eq!(71, solve_part2(&data)?))
    }
}
