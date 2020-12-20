extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

#[derive(Parser)]
#[grammar = "rule.pest"]
struct RuleParser;

#[derive(Debug)]
enum MessageRule {
    // Outer vec is the choices, inner vec is the sequence
    // of rule numbers within each choice branch.
    Choice(Vec<Vec<u8>>),
    Char(String)
}

type RuleMap = HashMap<u8, MessageRule>;

impl MessageRule {
    fn parse(input: &str) -> (u8, Self) {
        let parsed_rule = RuleParser::parse(Rule::rule, &input).unwrap().next().unwrap(); 
        let mut rule_inner = parsed_rule.into_inner();
        let num = rule_inner.next().unwrap().as_str().parse().unwrap();
        let rule_contents = rule_inner.next().unwrap();
        let message_rule = match rule_contents.as_rule() {
            Rule::choice => MessageRule::Choice(rule_contents.into_inner().map(|seq|
                seq.into_inner().map(|rule_num| rule_num.as_str().parse().unwrap()).collect()
            ).collect()),
            Rule::char_quoted => MessageRule::Char(
                rule_contents.into_inner().next().unwrap().as_str().to_string()
            ),
            _ => unreachable!()
        };
        (num, message_rule)
    }

    // Returns None if no match, otherwise returns the slice following the match.
    fn check<'a>(self: &Self, rules: &RuleMap, remaining: &'a str) -> Option<&'a str> {
        match self {
            MessageRule::Choice(choices) =>
                'search: for sequence in choices.iter() {
                    let mut sequence_slice = remaining;
                    for rule_num in sequence.iter() {
                        if let Some(remaining_sequence_slice) = rules[&rule_num].check(&rules, sequence_slice) {
                            sequence_slice = remaining_sequence_slice;
                        } else {
                            continue 'search;
                        }
                    }
                    return Some(sequence_slice)
                }
            MessageRule::Char(ch) =>
                if remaining[0..1] == *ch {
                    return Some(&remaining[1..])
                } 
        }
        None
    }
}

fn check_inputs<'a>(rules: &RuleMap, inputs: &'a[String]) -> Vec<&'a String> {
    inputs.iter().filter_map(|input| {
        if let Some(remaining_slice) = rules[&0].check(&rules, input) {
            if remaining_slice.len() == 0 {
                return Some(input);
            }
        }
        None
    }).collect()
}

fn parse_content(content: &str) -> (RuleMap, Vec<String>) {
    let mut lines = content.trim().lines().map(|line| line.trim());
    let mut rules = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() { break; }
        let (num, rule) = MessageRule::parse(line);
        rules.insert(num, rule);
    }
    let inputs = lines.map(|s| s.to_string()).collect();
    (rules, inputs)
}

fn main() {
    let (rule_map, inputs) = parse_content(&fs::read_to_string("input.txt").unwrap());
    println!("Matches by initial rules: {}", check_inputs(&rule_map, &inputs).len());
}

#[test]
fn test_sample() {
    const SAMPLE: &str = r#"
    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"

    ababbb
    bababa
    abbbab
    aaabbb
    aaaabbb"#;

    let (rule_map, inputs) = parse_content(SAMPLE);
    assert_eq!(check_inputs(&rule_map, &inputs).len(), 2);
}
