mod day1;
mod day3;
mod day4;

pub trait Represent {
    fn get_result(&self) {}
}

fn main() {
    let day1 = day1::Day1::from(include_str!("../data/day1.txt"));
    let day3 = day3::Day3::from(include_str!("../data/day3.txt"));
    let day4 = day4::Day4::from(include_str!("../data/day4.txt"));

    day1.get_result();
    day3.get_result();
    day4.get_result();
}
