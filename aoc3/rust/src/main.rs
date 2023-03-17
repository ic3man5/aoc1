use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_priority = 0u64;
    let values = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for line in reader.lines() {
        if let Ok(rucksack_line) = line {
            let (first_compartment, second_compartment) = rucksack_line.split_at(rucksack_line.len()/2);
            let second_compartment_set: HashSet<char> = second_compartment.chars().collect();
            let common_rucksacks: Vec<char> = first_compartment.chars().filter(|c| second_compartment_set.contains(&c)).collect();
            let priority = values.find(common_rucksacks[0]).unwrap() as u64 + 1;
            total_priority += priority;
            //println!("common: {first_compartment} {second_compartment} {common_rucksacks:?} {priority} {total_priority}");
        }
    }
    println!("Total Priority: {total_priority}");
}
