#[allow(dead_code)]
mod day1;
mod day2;
extern crate aoc_lib;

fn main() {
    //day1::run(aoc_lib::input_reader::get_input("2024", "1", "cookie.txt"));
    day2::run(aoc_lib::input_reader::get_input("2024", "2", "cookie.txt"));
}