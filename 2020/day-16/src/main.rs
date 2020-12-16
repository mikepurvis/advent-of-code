
#[macro_use]
extern crate lazy_static;

extern crate regex;


use regex::Regex;
use std::fs;
use std::ops::RangeInclusive;


#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<u16>>
}

impl Rule {
    fn parse(input: &str) -> Self {
        lazy_static! {
            // This regex just hard-codes the inputs "a-b or c-d" format, but it
            // would be easy to genericize it for arbitrary numbers of or-clauses.
            static ref RE: Regex = Regex::new(
                r"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
        }
        let caps = RE.captures(input).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let cap_num = |n| caps.get(n).unwrap().as_str().parse::<u16>().unwrap();
        let ranges = vec![
            cap_num(2)..=cap_num(3),
            cap_num(4)..=cap_num(5)
        ];
        Self { name, ranges }
    }

    fn check(self: &Self, value: u16) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

type Ticket = Vec<u16>;

fn parse_ticket(input: &str) -> Ticket {
    input.split(",").map(|s| {
        s.parse().unwrap()
    }).collect()
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut lines = input.trim().lines().map(|line| line.trim());
    let mut rules = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() { break; }
        rules.push(Rule::parse(line));
    }
    assert_eq!(lines.next().unwrap(), "your ticket:");
    let my_ticket = parse_ticket(lines.next().unwrap());

    assert_eq!(lines.next().unwrap(), "");
    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let tickets = lines.map(|line| {
        parse_ticket(line)
    }).collect();

    (rules, my_ticket, tickets)
}

// This would have been much much sensibly done either as a borrow of the ticket
// slice (so moving the flatten call inside this function) or by collecting the
// flatten result externally and then getting a slice of that. However, this was
// good practice to understand what it looks like to generically pass an iterator
// and properly manage the lifetimes of the referenced items.
fn find_invalid_numbers<'a, I>(nums: I, rules: &[Rule]) -> Vec<u16> 
where I: Iterator<Item = &'a u16> {
    nums.filter_map(|&num| {
        for rule in rules {
            if rule.check(num) {
                return None
            }
        }
        Some(num)
    }).collect()
}

fn find_valid_tickets<'a>(tickets: &'a[Ticket], rules: &[Rule]) -> Vec<&'a Ticket> {
    tickets.iter().filter_map(|ticket| {
        if find_invalid_numbers(ticket.iter(), rules).is_empty() {
            Some(ticket)
        } else {
            None
        }
    }).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (rules, my_ticket, tickets) = parse_input(&input);
    let invalid = find_invalid_numbers(tickets.iter().flatten(), &rules);
    println!("Sum of invalid numbers: {}", invalid.iter().sum::<u16>());
}

#[test]
fn test_rule() {
    let rule = Rule::parse("test: 1-3 or 5-7");
    assert_eq!(rule.name, "test");
    assert_eq!(rule.check(3), true);
    assert_eq!(rule.check(4), false);
    assert_eq!(rule.check(5), true);
}

#[test]
fn test_sample() {
    const SAMPLE: &str = r#"
    class: 1-3 or 5-7
    row: 6-11 or 33-44
    seat: 13-40 or 45-50

    your ticket:
    7,1,14

    nearby tickets:
    7,3,47
    40,4,50
    55,2,20
    38,6,12"#;

    let (rules, my_ticket, tickets) = parse_input(SAMPLE);
    let invalid_nums = find_invalid_numbers(tickets.iter().flatten(), &rules);
    assert_eq!(invalid_nums.iter().sum::<u16>(), 71);

    let valid_tickets = find_valid_tickets(&tickets, &rules);
    assert_eq!(valid_tickets.len(), 1);

}
