use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::utils::AocError::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Signal {
    Low,
    High,
}

trait Module {
    fn process(&mut self, from: &str, signal: Signal) -> (String, Vec<String>, Signal);
}

#[derive(Debug, Clone)]
pub struct FlipFlop {
    name: String,
    on: bool,
    output: Vec<String>,
}

impl FromStr for FlipFlop {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(" -> ");
        let name = split
            .next()
            .context("No name found")?
            .chars()
            .skip(1)
            .collect::<String>();
        let output = split
            .next()
            .context("No output found")?
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let on = false;

        Ok(FlipFlop { name, on, output })
    }
}

impl Module for FlipFlop {
    fn process(&mut self, from: &str, signal: Signal) -> (String, Vec<String>, Signal) {
        if signal == Signal::Low {
            self.on = !self.on;
            (
                self.name.clone(),
                self.output.clone(),
                if self.on { Signal::High } else { Signal::Low },
            )
        } else {
            (self.name.clone(), vec![], Signal::Low)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conjunction {
    name: String,
    input: HashMap<String, Signal>,
    output: Vec<String>,
}

impl FromStr for Conjunction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(" -> ");
        let name = split
            .next()
            .context("No name found")?
            .chars()
            .skip(1)
            .collect::<String>();
        let input = HashMap::new();
        let output = split
            .next()
            .context("No output found")?
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Conjunction {
            name,
            input,
            output,
        })
    }
}

impl Module for Conjunction {
    fn process(&mut self, from: &str, signal: Signal) -> (String, Vec<String>, Signal) {
        self.input
            .entry(from.to_string())
            .and_modify(|s| *s = signal);
        (
            self.name.clone(),
            self.output.clone(),
            if self.input.iter().all(|(_, v)| *v == Signal::High) {
                Signal::Low
            } else {
                Signal::High
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    name: String,
    output: Vec<String>,
}

impl FromStr for Button {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(" -> ");
        let name = split.next().context("No name found")?.to_string();
        let output = split
            .next()
            .context("No output found")?
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Ok(Button { name, output })
    }
}

impl Module for Button {
    fn process(&mut self, from: &str, signal: Signal) -> (String, Vec<String>, Signal) {
        (self.name.clone(), self.output.clone(), signal)
    }
}

#[derive(Debug, Clone)]
pub struct Output;

impl Module for Output {
    fn process(&mut self, from: &str, signal: Signal) -> (String, Vec<String>, Signal) {
        ("output".to_string(), vec![], Signal::Low)
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<Machine> {
    //     let input = "broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a";
    //     let input = "broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output";
    let buttons = input
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|l| !l.starts_with("%") && !l.starts_with("&"))
        .map(Button::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")?;

    let flipflop = input
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|l| l.starts_with("%"))
        .map(FlipFlop::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")?;

    let mut conjunction = input
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|l| l.starts_with("&"))
        .map(Conjunction::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Error while parsing input")?;

    for i in 0..conjunction.len() {
        let f = &conjunction[i];
        let bs = buttons
            .iter()
            .filter(|b| b.output.contains(&f.name))
            .map(|b| b.name.clone());
        let fs = flipflop
            .iter()
            .filter(|b| b.output.contains(&f.name))
            .map(|b| b.name.clone());
        let cs = conjunction
            .iter()
            .filter(|b| b.output.contains(&f.name))
            .map(|b| b.name.clone());

        conjunction[i].input = bs
            .chain(fs)
            .chain(cs)
            .map(|n| (n, Signal::Low))
            .collect::<HashMap<String, Signal>>();
    }

    Ok(Machine::new(buttons, flipflop, conjunction))
}

#[derive(Debug, Clone)]
pub struct Machine {
    buttons: Vec<Button>,
    flipflops: Vec<FlipFlop>,
    conjunctions: Vec<Conjunction>,
    output: Output,
}

impl Machine {
    fn new(buttons: Vec<Button>, flipflops: Vec<FlipFlop>, conjunctions: Vec<Conjunction>) -> Self {
        let output = Output {};
        Machine {
            buttons,
            flipflops,
            conjunctions,
            output,
        }
    }

    fn get(&mut self, name: &str) -> Option<&mut dyn Module> {
        for i in 0..self.buttons.len() {
            if self.buttons[i].name == name {
                return Some(&mut self.buttons[i]);
            }
        }

        for i in 0..self.flipflops.len() {
            if self.flipflops[i].name == name {
                return Some(&mut self.flipflops[i]);
            }
        }

        for i in 0..self.conjunctions.len() {
            if self.conjunctions[i].name == name {
                return Some(&mut self.conjunctions[i]);
            }
        }

        Some(&mut self.output)
    }
}

fn press_button(
    machine: &mut Machine,
    observe: Option<(String, Signal)>,
) -> Result<((usize, usize), bool)> {
    let mut queue = VecDeque::new();
    queue.push_back(("".to_string(), "broadcaster".to_string(), Signal::Low));

    let mut count = (0, 0);
    let mut done = false;

    while let Some(q) = queue.pop_front() {
        let (from, name, signal) = q;

        if signal == Signal::Low {
            count = (count.0 + 1, count.1);
        } else {
            count = (count.0, count.1 + 1);
        }

        let m = machine
            .get(&name)
            .with_context(|| format!("Unknown module {}", name))?;

        let (from, next, next_signal) = m.process(&from, signal);

        for n in next.clone() {
            if let Some((name, signal)) = &observe {
                if &from == name && next_signal.clone() == *signal {
                    done = true;
                }
            }
            queue.push_back((from.clone(), n, next_signal.clone()));
        }
    }

    Ok((count, done))
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Machine) -> Result<usize> {
    let mut machine = input.clone();

    let mut count = (0, 0);
    for _ in 0..1_000 {
        let (add, _) = press_button(&mut machine, None)?;
        count.0 += add.0;
        count.1 += add.1;
    }

    Ok(count.0 * count.1)
}

fn find_cycle(machine: &mut Machine, node: String, signal: Signal) -> Option<usize> {
    let mut count = 0;
    let mut last = 0;

    loop {
        count += 1;
        let (_, done) = press_button(machine, Some((node.clone(), signal.clone()))).ok()?;

        if done {
            if last != 0 {
                return Some(count - last);
            } else {
                last = count;
            }
        }

        if count == 1_000_000 {
            break;
        }
    }

    None
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Machine) -> Result<usize> {
    let modules = ["jm", "rh", "jg", "hf"]
        .iter()
        .map(|m| {
            let mut machine = input.clone();
            find_cycle(&mut machine, m.to_string(), Signal::High)
                .with_context(|| format!("Could not find cycle for {}", m))
        })
        .collect::<Result<Vec<_>>>()?;

    let count = modules
        .iter()
        .fold(1, |acc, el| num::Integer::lcm(&acc, el));

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> &'static str {
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
    }

    fn sample2() -> &'static str {
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
    }

    fn input(input: &str) -> Result<Machine> {
        input_generator(input)
    }

    #[test]
    fn part1_sample1() -> Result<()> {
        let data = input(sample1())?;
        Ok(assert_eq!(32_000_000, solve_part1(&data)?))
    }

    #[test]
    fn part1_sample2() -> Result<()> {
        let data = input(sample2())?;
        Ok(assert_eq!(11_687_500, solve_part1(&data)?))
    }
}
