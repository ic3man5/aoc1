use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_priority = 0u64;
    let priority_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for line in reader.lines() {
        if let Ok(rucksack_line) = line {
            // Each rucksack has two compartments, each compartments have equal sizes
            let (first_compartment, second_compartment) = rucksack_line.split_at(rucksack_line.len()/2);
            // Lets find the common characters in each
            let second_compartment_set: HashSet<char> = second_compartment.chars().collect();
            let common_rucksacks: Vec<char> = first_compartment.chars().filter(|c| second_compartment_set.contains(&c)).collect();
            // Lets find the priority of the letter:
            // Lowercase item types a through z have priorities 1 through 26.
            // Uppercase item types A through Z have priorities 27 through 52.
            let priority = priority_list.find(common_rucksacks[0]).unwrap() as u64 + 1;
            total_priority += priority;
        }
    }
    println!("Total Priority: {total_priority}");
}
