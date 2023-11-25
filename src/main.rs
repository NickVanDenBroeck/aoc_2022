mod day1;
mod common;

fn main() -> std::io::Result<()> {
    day1::part1("./src/input/day1.txt");
    day1::part2("./src/input/day1.txt");

    Ok(())
}

