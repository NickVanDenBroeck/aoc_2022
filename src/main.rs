mod day1;
mod day2;
mod common;

fn main() -> std::io::Result<()> {
    day1::part1("./src/input/day1.txt");
    day1::part2("./src/input/day1.txt");
    day2::part1("./src/input/day2.txt");
    day2::part2("./src/input/day2.txt");

    Ok(())
}







