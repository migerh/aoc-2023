use anyhow::{Context, Error, Result};
use gauss_jordan_elimination::gauss_jordan_elimination_generic;
use std::str::FromStr;

type Coords = (i128, i128, i128);
type Coordsf = (f64, f64, f64);

fn parse_coords(s: &str) -> Result<Coords> {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"^(?P<x>-?\d+?),\s+(?P<y>-?\d+?),\s+(?<z>-?\d+?)$").unwrap();
    }

    let (x, y, z) = RE
        .captures(s)
        .and_then(|cap| {
            let x = cap.name("x").map(|v| v.as_str())?.to_string();
            let y = cap.name("y").map(|v| v.as_str())?.to_string();
            let z = cap.name("z").map(|v| v.as_str())?.to_string();

            Some((x, y, z))
        })
        .context("Error during coords parse")?;

    let x = x.parse::<i128>()?;
    let y = y.parse::<i128>()?;
    let z = z.parse::<i128>()?;

    Ok((x, y, z))
}

#[derive(Debug)]
pub struct Stone {
    pos: Coords,
    velocity: Coords,
}

impl FromStr for Stone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^(?P<pos>.*?)\s+@\s+(?P<vel>.*?)$").unwrap();
        }

        let (pos, vel) = RE
            .captures(s)
            .and_then(|cap| {
                let pos = cap.name("pos").map(|v| v.as_str())?.to_string();
                let vel = cap.name("vel").map(|v| v.as_str())?.to_string();

                Some((pos, vel))
            })
            .context("Error during line parse")?;

        let pos = parse_coords(&pos)?;
        let velocity = parse_coords(&vel)?;

        Ok(Stone { pos, velocity })
    }
}

impl Stone {
    fn intersect(&self, other: &Stone) -> Option<Coordsf> {
        let p11 = self.pos;
        let p12 = (
            self.pos.0 + self.velocity.0,
            self.pos.1 + self.velocity.1,
            self.pos.2 + self.velocity.2,
        );

        let p21 = other.pos;
        let p22 = (
            other.pos.0 + other.velocity.0,
            other.pos.1 + other.velocity.1,
            other.pos.2 + other.velocity.2,
        );

        let i0t = (p11.0 * p12.1 - p11.1 * p12.0) * (p21.0 - p22.0)
            - (p11.0 - p12.0) * (p21.0 * p22.1 - p21.1 * p22.0);
        let i0b = (p11.0 - p12.0) * (p21.1 - p22.1) - (p11.1 - p12.1) * (p21.0 - p22.0);

        let i1t = (p11.0 * p12.1 - p11.1 * p12.0) * (p21.1 - p22.1)
            - (p11.1 - p12.1) * (p21.0 * p22.1 - p21.1 * p22.0);
        let i1b = (p11.0 - p12.0) * (p21.1 - p22.1) - (p11.1 - p12.1) * (p21.0 - p22.0);

        // lines are parallel
        if i0b == 0 || i1b == 0 {
            return None;
        }

        Some((i0t as f64 / i0b as f64, i1t as f64 / i1b as f64, 0.0))
    }

    fn point_in_future(&self, p: Coordsf) -> bool {
        let selfpos = (self.pos.0 as f64, self.pos.1 as f64, self.pos.2 as f64);
        let selfvel = (
            self.velocity.0 as f64,
            self.velocity.1 as f64,
            self.velocity.2 as f64,
        );

        (p.0 - selfpos.0) / selfvel.0 > 0.0 && (p.1 - selfpos.1) / selfvel.1 > 0.0
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Result<Vec<Stone>> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Stone::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Stone]) -> Result<u32> {
    let len = input.len();
    let (min, max) = if len == 5 { (7.0, 27.0) } else { (200000000000000.0, 400000000000000.0) };

    let mut count = 0;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if let Some(inter) = input[i].intersect(&input[j]) {
                if !input[i].point_in_future(inter) {
                    continue;
                }

                if !input[j].point_in_future(inter) {
                    continue;
                }

                if inter.0 >= min && inter.1 >= min && inter.0 <= max && inter.1 <= max {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &[Stone]) -> Result<i128> {
    let a = &input[0];
    let b = &input[1];
    let c = &input[2];

    let p_a = (a.pos.0 as f64, a.pos.1 as f64, a.pos.2 as f64);
    let p_b = (b.pos.0 as f64, b.pos.1 as f64, b.pos.2 as f64);
    let p_c = (c.pos.0 as f64, c.pos.1 as f64, c.pos.2 as f64);

    let v_a = (
        a.velocity.0 as f64,
        a.velocity.1 as f64,
        a.velocity.2 as f64,
    );
    let v_b = (
        b.velocity.0 as f64,
        b.velocity.1 as f64,
        b.velocity.2 as f64,
    );
    let v_c = (
        c.velocity.0 as f64,
        c.velocity.1 as f64,
        c.velocity.2 as f64,
    );

    let mut m = vec![
        vec![
            0.0,
            v_b.2 - v_a.2,
            v_a.1 - v_b.1,
            0.0,
            p_a.2 - p_b.2,
            p_b.1 - p_a.1,
            -(p_b.1 * v_b.2 - p_b.2 * v_b.1 + p_a.2 * v_a.1 - p_a.1 * v_a.2),
        ],
        vec![
            v_a.2 - v_b.2,
            0.0,
            v_b.0 - v_a.0,
            p_b.2 - p_a.2,
            0.0,
            p_a.0 - p_b.0,
            -(p_b.2 * v_b.0 - p_b.0 * v_b.2 - p_a.2 * v_a.0 + p_a.0 * v_a.2),
        ],
        vec![
            v_b.1 - v_a.1,
            v_a.0 - v_b.0,
            0.0,
            p_a.1 - p_b.1,
            p_b.0 - p_a.0,
            0.0,
            -(p_b.0 * v_b.1 - p_b.1 * v_b.0 - p_a.0 * v_a.1 + p_a.1 * v_a.0),
        ],
        vec![
            0.0,
            v_c.2 - v_a.2,
            v_a.1 - v_c.1,
            0.0,
            p_a.2 - p_c.2,
            p_c.1 - p_a.1,
            -(p_c.1 * v_c.2 - p_c.2 * v_c.1 + p_a.2 * v_a.1 - p_a.1 * v_a.2),
        ],
        vec![
            v_a.2 - v_c.2,
            0.0,
            v_c.0 - v_a.0,
            p_c.2 - p_a.2,
            0.0,
            p_a.0 - p_c.0,
            -(p_c.2 * v_c.0 - p_c.0 * v_c.2 - p_a.2 * v_a.0 + p_a.0 * v_a.2),
        ],
        vec![
            v_c.1 - v_a.1,
            v_a.0 - v_c.0,
            0.0,
            p_a.1 - p_c.1,
            p_c.0 - p_a.0,
            0.0,
            -(p_c.0 * v_c.1 - p_c.1 * v_c.0 - p_a.0 * v_a.1 + p_a.1 * v_a.0),
        ],
    ];
    gauss_jordan_elimination_generic(&mut m);

    let result = -(m[0][6] + m[1][6] + m[2][6]);

    Ok(result.floor() as i128)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
    }

    fn input() -> Result<Vec<Stone>> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(2, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<()> {
        let data = input()?;
        Ok(assert_eq!(47, solve_part2(&data)?))
    }
}
