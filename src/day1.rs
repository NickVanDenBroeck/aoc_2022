use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

pub fn part1(file_path: &str) {
    let path = Path::new(file_path);

    if let Ok(file) = File::open(path) {
    let reader = BufReader::new(file);
    let mut max_calories = 0; // To store the maximum Calories found
    let mut current_calories = 0; // To store the current Elf's total Calories

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                // Blank line, check if the current Elf has more Calories
                max_calories = max_calories.max(current_calories);
                current_calories = 0; // Reset current Elf's Calories
            } else {
                // Parse the Calories from the line and add to the current Elf's total
                if let Ok(calories) = line.trim().parse::<i32>() {
                    current_calories += calories;
                }
            }
        }
    }

        // Check one more time after the loop in case the last Elf has the most Calories
        max_calories = max_calories.max(current_calories);

        // Print the result
        println!("Elf carrying the most Calories: {}", max_calories);
    } else {
        eprintln!("Error opening the input file.");
    }   
}    

pub fn part2(file_path: &str){
    let file = File::open(file_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut elf_calories: HashMap<usize, i32> = HashMap::new();
    let mut current_elf = 0;
    let mut current_calories = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            elf_calories.insert(current_elf, current_calories);
            current_calories = 0;
            current_elf += 1;
        } else {
            let calories: i32 = line.parse().expect("Failed to parse Calories");
            current_calories += calories;
        }
    }

    // Insert the last Elf's Calories
    elf_calories.insert(current_elf, current_calories);

    // Sort the elves by total Calories in descending order
    let mut elves: Vec<_> = elf_calories.iter().collect();
    elves.sort_by_key(|&(_, &calories)| std::cmp::Reverse(calories));

    // Calculate the total Calories carried by the top three Elves
    let total_top_three_calories: i32 = elves.iter().take(3).map(|&(_, &calories)| calories).sum();

    println!("Total Calories carried by the top three Elves: {}", total_top_three_calories);
}