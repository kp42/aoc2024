mod day1;

fn main() {
    let data = include_str!("../data/day1.txt");
    let day1 = day1::Day1::from(data);

    println!("{:#?}", day1.part1());
    println!("{:#?}", day1.part2());
}
