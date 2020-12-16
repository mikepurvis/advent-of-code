use itertools::zip;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::vec::Vec;


fn parse_contents(contents: &str) -> (u64, Vec<Option<u64>>) {
    let mut lines = contents.lines();
    (
        lines.next().unwrap().trim().parse().unwrap(),
        parse_list(&lines.next().unwrap())
    )
}

fn parse_list(list: &str) -> Vec<Option<u64>> {
    list.split(',').map(|s| {
        match s.trim() {
            "x" => None,
            v => Some(v.parse().unwrap())
        }
    }).collect()
}

fn in_service_buses(all_buses: &[Option<u64>]) -> Vec<u64> {
    all_buses.iter().copied().filter_map(|x| x).collect()
}

fn next_buses(start_time: u64, buses: &[u64]) -> Vec<(u64, u64)> {
    buses.iter().map(|&bus| {
        let busn = start_time as f64 / bus as f64;
        let wait_time = ((busn.ceil() - busn) * bus as f64).round();
        (wait_time as u64, bus as u64)
    }).collect::<Vec<_>>()
}

// This solution was cute, but not nearly performant enough.
#[allow(dead_code)] 
fn find_sequence_naive(buses: &[Option<u64>]) -> u64 {
    let in_service = in_service_buses(&buses);

    // This infinite iterator yields a hashmap of all bus departures per time increment.
    let mut schedule = (1..).map(|t| {
        in_service.iter().copied().filter_map(|b| {
            if t % b == 0 { Some(b) } else { None }
        }).collect::<HashSet<u64>>()
    });

    let mut scanner: VecDeque<_> = schedule.by_ref().take(buses.len()).collect();

    'search: for time in 1.. {
        for (check_bus, scheduled_buses) in zip(buses, &scanner) {
            if let Some(bus) = check_bus {
                if !scheduled_buses.contains(bus) {
                    scanner.pop_front();
                    scanner.push_back(schedule.next().unwrap());
                    continue 'search;
                }
            }
        }
        return time;
    }
    unreachable!();
}

// This version goes contraint by constraint, stepping through the search space
// by the product of all previous constraints to find instances of the run that
// are at least that long.
fn find_sequence(buses: &[Option<u64>]) -> u64 {
    let mut time = 0u64;
    let mut search_step = 1u64;
    for (bus_offset, bus_search_option) in buses.iter().enumerate() {
        if let Some(bus_search) = bus_search_option {
            for search_time in (time..).step_by(search_step as usize) {
                if (search_time + bus_offset as u64) % bus_search == 0 {
                    search_step *= bus_search;
                    time = search_time;
                    break;
                }
            }

        }
    }
    time
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let (start_time, all_buses) = parse_contents(&contents);
    let next = next_buses(start_time, &in_service_buses(&all_buses));
    let (wait_time, bus) = next.iter().min().unwrap();
    println!("Multiple: {:?}", wait_time * bus);

    let sequence_time = find_sequence(&all_buses);
    println!("Sequence Time: {:?}", sequence_time);
}


#[test]
fn test_sample() {
    const SAMPLE: &str = "939
    7,13,x,x,59,x,31,19";

    let (start_time, all_buses) = parse_contents(&SAMPLE);
    let buses = in_service_buses(&all_buses);
    let next = next_buses(start_time, &buses);
    let (wait_time, bus) = next.iter().min().unwrap();
    assert_eq!(wait_time * bus, 295);
    
    assert_eq!(find_sequence(&all_buses), 1068781);
    assert_eq!(find_sequence_naive(&all_buses), 1068781);
}

#[test]
fn more_samples() {
    assert_eq!(find_sequence(&parse_list("17,x,13,19")), 3417);
    assert_eq!(find_sequence(&parse_list("67,7,59,61")), 754018);
    assert_eq!(find_sequence(&parse_list("67,x,7,59,61")), 779210);
    assert_eq!(find_sequence(&parse_list("67,7,x,59,61")), 1261476);
    assert_eq!(find_sequence(&parse_list("1789,37,47,1889")), 1202161486);
}
