extern crate defaultmap;
extern crate itertools;

use defaultmap::DefaultHashMap;
use itertools::zip;
use std::fs;
use std::collections::HashMap;
use std::vec::Vec;

fn numbers_from_contents(contents: &str) -> Vec<u32> {
    contents.lines()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect()
}

fn get_sorted(input: &[u32]) -> Vec<u32> {
    let mut sorted = input.to_vec();

    // Add the airplane socket before sorting; it will get sorted to
    // the beginning of the list, then add the device socket after,
    // when we know what the highest value is.
    sorted.push(0);
    sorted.sort();
    sorted.push(sorted.last().unwrap() + 3);
    return sorted;
}

fn count_jolt_differences(sorted: &[u32]) -> HashMap<u32, u32> {
    let mut counts = DefaultHashMap::<u32, u32>::new(0);
    for (a, b) in zip(sorted, &sorted[1..]) {
        counts[b - a] += 1;
    }
    counts.into()
}

fn count_jolt_permutations(sorted: &[u32]) -> u64 {
    // Recursive, slow search. Do this for each run where there 
    // are deltas of 1 and 2 between the items.
    fn _count(list: &[u32]) -> u64 {
        let upper = if list.len() > 4 { 4 }
                    else if list.len() > 1 { list.len() }
                    else { return 1; };
        let mut options = 0;
        for index in 1..upper {
            if list[index] - list[0] > 3 {
                break;
            }
            options += _count(&list[index..]);
        }
        return options;
    }

    // Break up the overall sequence into runs by finding the diffs
    // of three, run the recursive logic over those segments and get
    // the product of all of it.
    let mut perms = 1u64;
    let mut base_index = 0;
    while base_index < sorted.len() - 2 {
        for index in base_index..(sorted.len() - 1) {
            if sorted[index + 1] - sorted[index] == 3 {
                perms *= _count(&sorted[base_index..(index + 1)]);
                base_index = index + 1;
                break;
            }
        }
    }
    return perms;
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let sorted = get_sorted(&numbers_from_contents(&contents));
    let diffs = count_jolt_differences(&sorted);
    println!("Multiple: {}", diffs[&1] * diffs[&3]);

    let perms = count_jolt_permutations(&sorted);
    println!("Permutations: {}", perms); 
}

#[test]
fn test_sample1()
{
    const SAMPLE: &str = r#"16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4"#;

    let sorted = get_sorted(&numbers_from_contents(SAMPLE));
    let diffs = count_jolt_differences(&sorted);
    assert_eq!(diffs[&1], 7);
    assert_eq!(diffs[&3], 5);

    let perms = count_jolt_permutations(&sorted);
    assert_eq!(perms, 8);
}

#[test]
fn test_sample2()
{
    const SAMPLE: &str = r#"28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"#;

    let sorted = get_sorted(&numbers_from_contents(SAMPLE));
    let diffs = count_jolt_differences(&sorted);
    assert_eq!(diffs[&1], 22);
    assert_eq!(diffs[&3], 10);

    let perms = count_jolt_permutations(&sorted);
    assert_eq!(perms, 19208);
}
