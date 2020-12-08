extern crate itertools;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use itertools::Itertools;
use pest::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Parser)]
#[grammar = "bags.pest"]
struct BagsParser;

type BagRule = HashSet<(usize, String)>;
type BagRuleMap = HashMap<String, BagRule>;

fn bag_rule_map_from_contents(contents: &str) -> BagRuleMap {
    fn get_bag_type(rule: Pair<'_, Rule>) -> String {
        assert_eq!(rule.as_rule(), Rule::bag_type);
        let mut inner = rule.into_inner();
        let mut result = "".to_string();
        result.push_str(inner.next().unwrap().as_str());
        result.push_str(" ");
        result.push_str(inner.next().unwrap().as_str());
        return result;
    }

    let mut bag_rule_map: BagRuleMap = HashMap::new();
	let file = BagsParser::parse(Rule::file, &contents).unwrap().next().unwrap();
    for statement in file.into_inner() {
        if statement.as_rule() == Rule::statement {
            let mut inner = statement.into_inner();
            let containing_bag_type = get_bag_type(inner.next().unwrap());
            let bag_contents = inner.next().unwrap();
            if bag_contents.as_rule() == Rule::contents {
                let mut bag_rule: BagRule = HashSet::new();
                for mut chunk in &bag_contents.into_inner().chunks(2) {
                    let num = &chunk.next().unwrap().as_str().parse::<usize>().unwrap();
                    let bag_type = get_bag_type(chunk.next().unwrap());
                    bag_rule.insert((*num, bag_type.to_string()));
                }
                bag_rule_map.insert(containing_bag_type.to_string(), bag_rule);
            }
        }
    }
    return bag_rule_map;
}

fn invert_rule_map(rule_map: &BagRuleMap) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for (bag, contents) in rule_map.iter() {
        for (_q, contents_bag) in contents.iter() {
            if !map.contains_key(&contents_bag.to_string()) {
                map.insert(contents_bag.to_string(), HashSet::new());
            }
            map.get_mut(contents_bag).unwrap().insert(bag.to_string());
        }
    }
    return map;
}

fn inverse_find(inverse_map: &HashMap<String, HashSet<String>>, needle: String) -> HashSet<String> {
    let mut set = HashSet::new();
    match inverse_map.get(&needle) {
        Some(found) => for bag in found.iter() {
            set.insert(bag.to_string());
            set.extend(inverse_find(inverse_map, bag.to_string()));
        },
        None => ()
    }
    return set;
}

fn main() {
	let contents = fs::read_to_string("input.txt").unwrap();
    let rule_map = bag_rule_map_from_contents(&contents);
    let inverse = invert_rule_map(&rule_map);
    let found = inverse_find(&inverse, "shiny gold".to_string());
    println!("Found bags: {}", found.len());
}


#[test]
fn test_sample() {
    const SAMPLE_INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;
    let rule_map = bag_rule_map_from_contents(&SAMPLE_INPUT);
    let inverse = invert_rule_map(&rule_map);
    let found = inverse_find(&inverse, "shiny gold".to_string());
    assert_eq!(4, found.len());
}
