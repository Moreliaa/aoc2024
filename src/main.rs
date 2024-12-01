#[allow(dead_code)]
mod day1;
extern crate aoc_lib;

fn main() {
    let year = "2024";
    let day = "1";
    let input = aoc_lib::input_reader::get_input(year, day, "cookie.txt");
    day1::run(input);
}