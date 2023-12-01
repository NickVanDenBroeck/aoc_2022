use std::fs::{File};
use std::io::{BufRead, BufReader, self};

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: Vec::new() }
    }

    fn push(&mut self, crate_id: char) {
        self.crates.push(crate_id);
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }
}

pub fn part1(file_path: &str) -> io::Result<()> {
    // Open the input file
    let file: File = File::open(file_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut stacks = vec![Stack::new(); 3]; // Create three empty stacks

    let mut reading_moves = false;

    // Read and process the lines from the file
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            // Skip empty lines
            continue;
        }
        if line.starts_with("move") {
            // Switch to reading move instructions
            reading_moves = true;
        } else if !reading_moves {
            // Process the initial configuration
            for (stack_number, crate_id) in line.chars().enumerate() {
                stacks[stack_number].push(crate_id);
            }
        } else {
            // Process move instructions
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 6 {
                if let (Ok(from_stack), Ok(to_stack)) = (
                    parts[3].parse::<usize>(),
                    parts[5].parse::<usize>(),
                ) {
                    if let Some(crate_to_move) = stacks[from_stack - 1].pop() {
                        stacks[to_stack - 1].push(crate_to_move);
                    }
                }
            }
        }
    }

    // Print the result
    for (stack_number, stack) in stacks.iter().enumerate() {
        if let Some(top_crate) = stack.crates.last() {
            println!("Stack {}: {}", stack_number + 1, top_crate);
        }
    }

    Ok(())
}