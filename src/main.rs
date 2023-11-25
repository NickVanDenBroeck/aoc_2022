mod day1;
mod day4;
mod common;

fn main() -> std::io::Result<()> {
    day1::part1("./src/input/day1.txt");
    day1::part2("./src/input/day1.txt");
    day4::part1("./src/input/day2.txt");
    day4::part2("./src/input/day2.txt");

    Ok(())
}







