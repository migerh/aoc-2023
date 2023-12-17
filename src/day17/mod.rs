use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;
use pathfinding::prelude::dijkstra;

use crate::utils::AocError::*;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u32>>> {
//     let input = "2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533";
    Ok(input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec())
}

fn turn_left(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        _ => panic!("No way"),
    }
}

fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, -1) => (1, 0),
        _ => panic!("No way"),
    }
}

fn is_inside(pos: (i32, i32), width: usize, height: usize) -> bool {
    pos.0 >= 0 && (pos.0 as usize) < width && pos.1 >= 0 && (pos.1 as usize) < height
}

fn trace(map: &[Vec<u32>]) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let start = (0, 0);
    let dir = (1, 0);
    let result = dijkstra(&(start, dir, 0), |node| {
        let mut neighbors = vec![];
        let (pos, dir, straight) = node;
        let loss = map[pos.1 as usize][pos.0 as usize];

        if *straight < 2 {
            let forward = ((pos.0 + dir.0, pos.1 + dir.1), *dir, straight + 1);
            if is_inside(forward.0, width, height) {
                neighbors.push((forward, loss));
            }
        }

        let left_dir = turn_left(*dir);
        let left_pos = (pos.0 + left_dir.0, pos.1 + left_dir.1);
        if is_inside(left_pos, width, height) {
            neighbors.push(((left_pos, left_dir, 0), loss));
        }
        
        let right_dir = turn_right(*dir);
        let right_pos = (pos.0 + right_dir.0, pos.1 + right_dir.1);
        if is_inside(right_pos, width, height) {
            neighbors.push(((right_pos, right_dir, 0), loss));
        }

        neighbors
    }, |pos| pos.0.0 as usize == width - 1 && pos.0.1 as usize == height - 1)
    // remove
    .unwrap();

    // for i in result.0.clone() {
    //     println!("{:?}\n", i);
    // }

    // let mut visual = map.to_vec();
    // result.0.iter().for_each(|node| {
    //     let (pos, _, _) = node;
    //     visual[pos.1 as usize][pos.0 as usize] = 0;
    // });
    // for y in 0..height {
    //     println!("{}", visual[y].iter().map(|c| c.to_string()).join(""));
    // }

    // dijkstra adds the weight of the first one but omits the weight of the last node
    // but we want it the other way round so adjust for that
    result.1 - map[0][0] + map[height - 1][width - 1]
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> Result<u32> {
    println!("{:?}", input);

    let result = trace(input);

    Ok(result)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> Result<u32> {
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
