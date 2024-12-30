use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    character::{complete::line_ending, is_newline},
    combinator::all_consuming,
    multi::{many0, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    match Puzzle::try_from(input) {
        Ok(puzzle) => {
            let valid_updates = puzzle.get_valid_updates();
            Some(
                valid_updates
                    .into_iter()
                    .filter_map(|update| update.get(update.len() / 2))
                    .sum::<u32>(),
            )
        }
        Err(_) => {
            dbg!("Parsing Error");
            None
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct Rules {
    rule_list: HashMap<u32, HashSet<u32>>,
}

struct Puzzle {
    rules: Rules,
    updates: Vec<Vec<u32>>,
}

#[derive(PartialEq)]
enum UpdateState {
    Valid,
    Invalid,
}

impl Puzzle {
    fn check_update(&self, update: &Vec<u32>) -> UpdateState {
        let updated_pages = update.clone().into_iter().collect::<HashSet<u32>>();
        let mut restricted_rules = self.rules.rule_list.clone();
        restricted_rules.retain(|k, _| updated_pages.contains(k));
        restricted_rules.iter_mut().for_each(|(_, hs)| {
            hs.retain(|after| updated_pages.contains(after));
        });

        let mut seen_set: HashSet<u32> = HashSet::new();
        for update_item in update {
            if let Some(applicable_rules) = restricted_rules.get(&update_item) {
                if seen_set.iter().any(|seen| applicable_rules.contains(seen)) {
                    return UpdateState::Invalid;
                }
            }
            seen_set.insert(*update_item);
        }
        return UpdateState::Valid;
    }

    fn get_valid_updates(&self) -> Vec<&Vec<u32>> {
        self.updates
            .iter()
            .filter_map(|update| {
                (self.check_update(update) == UpdateState::Valid).then_some(update)
            })
            .collect_vec()
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_puzzle(value) {
            Ok((_, puzzle)) => Ok(puzzle),
            Err(_) => Err("Error parsing puzzle."),
        }
    }
}

fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(
        nom::character::complete::u32,
        tag("|"),
        nom::character::complete::u32,
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), nom::character::complete::u32)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, parse_update)(input)
}

fn parse_puzzle(input: &str) -> IResult<&str, Puzzle> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let result = parse_updates(input);
    let (input, updates) = result?;
    let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules.iter().for_each(|(before, after)| {
        rule_map.entry(*before).or_default().insert(*after);
    });
    Ok((
        input,
        Puzzle {
            rules: Rules {
                rule_list: rule_map,
            },
            updates,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
