extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::{HashMap, HashSet};
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

    // Returns the possible slices following the match or an empty set for no matches down this
    // branch of the search tree.
    fn check<'a>(self: &Self, rules: &RuleMap, remaining: &'a str) -> HashSet<&'a str> {
        let mut remainders = HashSet::<&str>::new();
        println!("> {:?} {:?}", &self, &remaining);
        if remaining.len() > 0 {
            match self {
                MessageRule::Choice(choices) => {
                    for sequence in choices.iter() {
                        //remainders.extend(&MessageRule::check_sequence(
                        //        &rules, &sequence, remaining));
                        let s = MessageRule::check_sequence(&rules, &sequence, remaining);

                        remainders.extend(&s);
                        println!("xx {:?} {:?}", &remainders, &s);
                    }
                }
                MessageRule::Char(ch) =>
                    if remaining[0..1] == *ch {
                        remainders.insert(&remaining[1..]);
                    } 
            }
        }
        println!("< {:?} {:?}", &self, &remainders);
        return remainders;
    }

    // Similar to the above, but instead of taking a rule, this one takes a slice into a
    // sequence. This is needed as a separate recursive function because there is branching
    // possible within a sequence.
    fn check_sequence<'a>(rules: &RuleMap, seq_steps: &[u8], remaining: &'a str) -> HashSet<&'a str> {
        let mut remainders = HashSet::<&str>::new();
        let rule_num = seq_steps[0];
        let all_remaining_after_rule = rules[&rule_num].check(&rules, &remaining);
        println!("{:?} {:?} {:?}", &seq_steps, &remaining, &all_remaining_after_rule);
        for remaining_after_rule in all_remaining_after_rule.iter() {
            if seq_steps.len() > 1 && remaining_after_rule.len() > 1 {
                let s = MessageRule::check_sequence(&rules, &seq_steps[1..], &remaining_after_rule);
                remainders.extend(&s);
                println!("xxx {:?} {:?}", &remainders, &s);
            } else if seq_steps.len() == 1 && remaining_after_rule.len() == 1 {
                remainders.insert(&"");
            }
        }
        return remainders;
    }
}

fn check_inputs<'a>(rules: &RuleMap, inputs: &'a[String]) -> Vec<&'a String> {
    inputs.iter().filter_map(|input| {
        let remaining_slices = rules[&0].check(&rules, input);
        if remaining_slices.contains(&"") {
            return Some(input);
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

fn update_rules(rule_map: &mut RuleMap) {
    let updates = vec![
        "8: 42 | 42 8",
        "11: 42 31 | 42 11 31"
    ];
    for rule_str in updates {
        let (num, rule) = MessageRule::parse(rule_str);
        rule_map.insert(num, rule);
    }
}

fn main() {
    let (mut rule_map, inputs) = parse_content(&fs::read_to_string("input.txt").unwrap());
    println!("Matches by initial rules: {}", check_inputs(&rule_map, &inputs).len());

    update_rules(&mut rule_map);
}

#[test]
fn test_sample1() {
    const SAMPLE: &str = r#"
    0: 4 1 5
    1: 2 3 | 3 2
    2: 4 4 | 5 5
    3: 4 5 | 5 4
    4: "a"
    5: "b"

    ababbb"#;
    /*bababa
    abbbab
    aaabbb
    aaaabbb"#;*/

    let (rule_map, inputs) = parse_content(SAMPLE);
    assert_eq!(check_inputs(&rule_map, &inputs).len(), 2);
}

#[test]
fn test_sample2() {
    const SAMPLE: &str = r#"
    42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: "a"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: "b"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1

    abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    bbabbbbaabaabba
    babbbbaabbbbbabbbbbbaabaaabaaa
    aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    bbbbbbbaaaabbbbaaabbabaaa
    bbbababbbbaaaaaaaabbababaaababaabab
    ababaaaaaabaaab
    ababaaaaabbbaba
    baabbaaaabbaaaababbaababb
    abbbbabbbbaaaababbbbbbaaaababb
    aaaaabbaabaaaaababaa
    aaaabbaaaabbaaa
    aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    babaaabbbaaabaababbaabababaaab
    aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    let (mut rule_map, inputs) = parse_content(SAMPLE);
    assert_eq!(check_inputs(&rule_map, &inputs).len(), 3);

    update_rules(&mut rule_map);
    println!("{:?}", check_inputs(&rule_map, &inputs));

    //assert_eq!(check_inputs(&rule_map, &inputs).len(), 12);
}

#[test]
fn test_sample3() {
    const SAMPLE: &str = r#"
    42: 9 14 | 10 1
    9: 14 27 | 1 26
    10: 23 14 | 28 1
    1: "a"
    11: 42 31
    5: 1 14 | 15 1
    19: 14 1 | 14 14
    12: 24 14 | 19 1
    16: 15 1 | 14 14
    31: 14 17 | 1 13
    6: 14 14 | 1 14
    2: 1 24 | 14 4
    0: 8 11
    13: 14 3 | 1 12
    15: 1 | 14
    17: 14 2 | 1 7
    23: 25 1 | 22 14
    28: 16 1
    4: 1 1
    20: 14 14 | 1 15
    3: 5 14 | 16 1
    27: 1 6 | 14 18
    14: "b"
    21: 14 1 | 1 14
    25: 1 1 | 1 14
    22: 14 14
    8: 42
    26: 14 22 | 1 20
    18: 15 15
    7: 14 5 | 1 21
    24: 14 1

    aaaaabbaabaaaaababaa"#;

    let (mut rule_map, inputs) = parse_content(SAMPLE);

    update_rules(&mut rule_map);
    println!("{:?}", check_inputs(&rule_map, &inputs));
}
