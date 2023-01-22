use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
  
fn main() {
    // Open the file
    let file = File::open("calories.txt").unwrap();
    let reader = BufReader::new(file);
    // read all the calories in the file
    let mut calories_total: u64 = 0;
    let mut all_calories = Vec::<u64>::new();
    for line in reader.lines() {
        if let Ok(cals) = line {
            if cals.is_empty() {
                all_calories.push(calories_total);
                calories_total = 0;
                continue;
            }
            calories_total += cals.parse::<u64>().unwrap();
        }
    }
    //println!("Number of Elfs: {}", all_calories.len());
    // Display which elf had the highest amount of total calories
    if let Some(cal) = all_calories.iter().max() {
        println!("Elf with the highest calorie count is: {cal}!");
    } else {
        panic!("Error: Calories list is empty!");
    }
}
