use day01::part1::process;

pub fn main() {
    let file = include_str!("../../part1.txt");
    let result = process(file);

    println!("Result is {}", result);
}
