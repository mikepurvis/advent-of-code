use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Group {
    people: Vec<HashSet<char>>,
}

impl Group {
    fn from_lines(mut lines: &mut std::str::Lines) -> Option<Self> {
        let mut group = Self { people: Vec::new() };
        for line in &mut lines {
            if line.trim().is_empty() {
                // Found a linebreak, this group is done.
                break;
            }
            group.people.push(line.chars().collect());
        }
        if group.people.is_empty() {
            return None;
        } 
        Some(group)
    }

    fn union(&self) -> HashSet<char> {
        self.people.iter().flatten().copied().collect()
    }

    fn intersection(&self) -> HashSet<char> {
        self.people.iter().fold(self.people[0].clone(),
            |h, other| h.intersection(&other).copied().collect())
    }
}

type Groups = Vec<Group>;

fn groups_from_str(contents: &String) -> Groups {
    let mut lines = contents.lines();
    let mut groups = Vec::new();
    loop {
        match Group::from_lines(&mut lines) {
            Some(group) => groups.push(group),
            None => break
        }
    }
    return groups
}

fn group_union_lens(groups: &Groups) -> Vec<usize> {
    groups.iter().map(|g| g.union().len()).collect()
}

fn group_intersection_lens(groups: &Groups) -> Vec<usize> {
    groups.iter().map(|g| g.intersection().len()).collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let groups = groups_from_str(&contents);
    println!("Union sum: {}", group_union_lens(&groups).iter().sum::<usize>());
    println!("Intersection sum: {}", group_intersection_lens(&groups).iter().sum::<usize>());
}

#[test]
fn test_sample() {
    const SAMPLE_INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;

    let groups = groups_from_str(&SAMPLE_INPUT.to_string());
    assert_eq!(group_union_lens(&groups), vec![3, 3, 3, 1, 1]);
    assert_eq!(group_intersection_lens(&groups), vec![3, 0, 1, 1, 1]);
}
