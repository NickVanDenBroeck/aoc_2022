
use std::fs;

pub fn part1(file_path: &str) {
    // Read the input data from a file (adjust the filename as needed).
    let input = fs::read_to_string(file_path).expect("Failed to read input file");

    // Split the input into lines and process each line.
    let mut count = 0;
    for line in input.lines() {
        // Split each line into two section assignments.
        let assignments: Vec<&str> = line.split(",").collect();

        // Parse the section assignments into ranges.
        let range1: Vec<u32> = parse_range(assignments[0]);
        let range2: Vec<u32> = parse_range(assignments[1]);

        // Check if one range fully contains the other.
        if (range1[0] <= range2[0] && range1[1] >= range2[1]) ||
        (range2[0] <= range1[0] && range2[1] >= range1[1]) {
            count += 1;
        }
    }

    println!("Number of assignment pairs with full containment: {}", count);
}

// Parse a section assignment string into a range of section IDs.
fn parse_range(assignment: &str) -> Vec<u32> {
    let parts: Vec<&str> = assignment.split('-').collect();
    let start: u32 = parts[0].parse().expect("Failed to parse start");
    let end: u32 = parts[1].parse().expect("Failed to parse end");
    vec![start, end]
}


pub fn part2(file_path: &str) {
    // Read the input data and split it into pairs of assignments
    let input = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();
    let assignments: Vec<(&str, &str)> = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        (parts[0], parts[1])
    }).collect();

    // Initialize a counter for overlapping assignment pairs
    let mut overlap_count = 0;

    // Iterate through each pair of assignments and check for overlap
    for i in 0..assignments.len() {        
        let part1: &str = assignments[i].0;
        let part2: &str = assignments[i].1;

        let assignment1 = parse_assignment(part1);
        let assignment2 = parse_assignment(part2);

        if overlap(assignment1, assignment2) {
            overlap_count += 1;
        }        
    }

    println!("Number of overlapping assignment pairs: {}", overlap_count);
}

// Function to check if two assignments overlap
fn overlap(assignment1: (u32, u32), assignment2: (u32, u32)) -> bool {
    let (start1, end1) = assignment1;
    let (start2, end2) = assignment2;
    start1 <= end2 && start2 <= end1
}

// Function to parse an assignment string into a tuple of (start, end)
fn parse_assignment(assignment: &str) -> (u32, u32) {
    let values: Vec<u32> = assignment.split('-').map(|x| x.parse().unwrap()).collect();
    (values[0], values[1])
}