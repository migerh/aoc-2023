use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::{str::FromStr, collections::HashMap};

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^\{x=(?P<x>\d+),m=(?P<m>\d+?),a=(?P<a>\d+?),s=(?P<s>\d+?)\}$").unwrap();
        }

        let (x, m, a, s) = RE
            .captures(s)
            .and_then(|cap| {
                let x = cap.name("x").map(|v| v.as_str())?.to_string();
                let m = cap.name("m").map(|v| v.as_str())?.to_string();
                let a = cap.name("a").map(|v| v.as_str())?.to_string();
                let s = cap.name("s").map(|v| v.as_str())?.to_string();

                Some((x, m, a, s))
            })
            .context("Error during parse")?;

        let x = x.parse::<u64>()?;
        let m = m.parse::<u64>()?;
        let a = a.parse::<u64>()?;
        let s = s.parse::<u64>()?;

        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug, Clone)]
pub enum Next {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Next {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "A" => Next::Accept,
            "R" => Next::Reject,
            v => Next::Workflow(v.to_string()),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    X,
    M,
    A,
    S,
}

impl FromStr for Property {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => Err(GenericError).context("Error parsing property")?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Rule {
    Less(Property, u64, Next),
    Greater(Property, u64, Next),
    Else(Next),
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref CHECK: regex::Regex =
                regex::Regex::new(r"^(?P<property>[xmas])(?P<compare>[<>])(?P<num>\d+?):(?P<next>\w+?)$").unwrap();
            static ref FALLBACK: regex::Regex =
                regex::Regex::new(r"^(?P<next>\w+?)$").unwrap();
        }

        if let Some((property, compare, num, next)) = CHECK
            .captures(s)
            .and_then(|cap| {
                let property = cap.name("property").map(|v| v.as_str())?.to_string();
                let compare = cap.name("compare").map(|v| v.as_str())?.to_string();
                let num = cap.name("num").map(|v| v.as_str())?.to_string();
                let next = cap.name("next").map(|v| v.as_str())?.to_string();

                Some((property, compare, num, next))
            }) {

            let property = Property::from_str(&property)?;
            let num = num.parse::<u64>()?;
            let next = Next::from_str(&next)?;

            if compare == "<" {
                Ok(Rule::Less(property, num, next))
            } else if compare == ">" {
                Ok(Rule::Greater(property, num, next))
            } else {
                Err(GenericError).context("Unknown comparison")?
            }
        } else {
            let next = FALLBACK
                .captures(s)
                .and_then(|cap| {
                    let next = cap.name("next").map(|v| v.as_str())?.to_string();

                    Some(next)
                }).context("Could not parse fallback")?;

            let next = Next::from_str(&next)?;

            Ok(Rule::Else(next))
        }

    }
}

#[derive(Debug, Clone)]
pub struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^(?P<id>\w+?)\{(?P<workflows>.*?)\}$").unwrap();
        }

        let (id, rules) = RE
            .captures(s)
            .and_then(|cap| {
                let id = cap.name("id").map(|v| v.as_str())?.to_string();
                let rules = cap.name("workflows").map(|v| v.as_str())?.to_string();

                Some((id, rules))
            })
            .context("Error during parse")?;

        let rules = rules.split(",").map(Rule::from_str).collect::<Result<Vec<_>>>()?;

        Ok(Workflow { id, rules })
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<(Vec<Workflow>, Vec<Part>)> {
//     let input = "px{a<2006:qkq,m>2090:A,rfg}
// pv{a>1716:R,A}
// lnx{m>1548:A,A}
// rfg{s<537:gd,x>2440:R,A}
// qs{s>3448:A,lnx}
// qkq{x<1416:A,crn}
// crn{x>2662:A,R}
// in{s<1351:px,qqz}
// qqz{s>2770:qs,m<1801:hdj,R}
// gd{a>3333:R,R}
// hdj{m>838:A,pv}
// 
// {x=787,m=2655,a=1222,s=2876}
// {x=1679,m=44,a=2067,s=496}
// {x=2036,m=264,a=79,s=2244}
// {x=2461,m=1339,a=466,s=291}
// {x=2127,m=1623,a=2188,s=1013}";
    let mut split = input
        .split("\n\n");

    let workflows = split.next().context("No workflows")?.lines().map(Workflow::from_str).collect::<Result<Vec<_>>>()?;
    let parts = split.next().context("No parts")?.lines().map(Part::from_str).collect::<Result<Vec<_>>>()?;

    Ok((workflows, parts))
}

impl Rule {
    pub fn next(&self, part: &Part) -> Option<&Next> {
        println!("Checking {:?} against {:?}", part, self);
        match self {
            Rule::Greater(prop, num, next) => {
                match prop {
                    Property::X if part.x > *num => Some(next),
                    Property::M if part.m > *num => Some(next),
                    Property::A if part.a > *num => Some(next),
                    Property::S if part.s > *num => Some(next),
                    _ => None,
                }
            },
            Rule::Less(prop, num, next) => {
                match prop {
                    Property::X if part.x < *num => Some(next),
                    Property::M if part.m < *num => Some(next),
                    Property::A if part.a < *num => Some(next),
                    Property::S if part.s < *num => Some(next),
                    _ => None,
                }
            }
            Rule::Else(n) => Some(n),
        }
    }
}

impl Part {
    pub fn rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    pub fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        println!("checking {:?}", self);
        if let Some(in_workflow) = workflows.get("in") {
            let mut workflow = in_workflow.clone();

            loop {
                println!();
                println!("workflow: {}", workflow.id);
                for rule in workflow.rules.clone() {
                    println!("Rule {:?}", rule);
                    if let Some(next) = rule.next(self) {
                        match next {
                            Next::Accept => return true,
                            Next::Reject => {
                                println!("Straight up reject");
                                return false
                            },
                            Next::Workflow(wf) => {
                                println!("Next workflow {:?}", wf);
                                if let Some(w) = workflows.get(wf) {
                                    workflow = w.clone();
                                    break;
                                } else {
                                    println!("Workflow {} not found", wf);
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Ran out of rules?");
        false
    }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &(Vec<Workflow>, Vec<Part>)) -> Result<u64> {
    let (workflows, parts) = input;
    let workflows = workflows.iter().cloned().map(|w| (w.id.clone(), w)).collect::<HashMap<String, Workflow>>();

    let rating = parts.iter().filter_map(|p| if p.is_accepted(&workflows) {
        Some(p.rating())
    } else {
        None
    }).sum();

    Ok(rating)
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &(Vec<Workflow>, Vec<Part>)) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Part>> {
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
