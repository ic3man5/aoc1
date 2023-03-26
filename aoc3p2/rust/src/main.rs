use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_priority = 0u64;
    let priority_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut lines = reader.lines();
    //let mut rucksacks = Vec::new();
    'main: loop {
        // Collect three rucksacks
        let mut rucksacks = Vec::new();
        for _ in 0..3 {
            let line = match lines.next() {
                Some(Ok(s)) => s,
                Some(Err(_)) => break 'main,
                None => break 'main,
            };
            rucksacks.push(line);
        };
        let [first, second, third] = &rucksacks[0..=2] else { todo!() };
        //println!("{first} {second} {third}");
        let second_set: HashSet<char> = second.chars().collect();
        let third_set: HashSet<char> = third.chars().collect();
        let common_rucksacks: Vec<char> = 
            first
            .chars()
            .filter(
                |c| {
                    second_set.contains(&c) && third_set.contains(&c)
                })
            .collect();
        //println!("{:?}", common_rucksacks);
        let priority = priority_list.find(common_rucksacks[0]).unwrap() as u64 + 1;
        total_priority += priority;
    };
    println!("Total Priority: {total_priority}");
}
