use regex::Regex;

use crate::Represent;

#[derive(Debug, Default)]
pub struct Day3 {
    value: String,
}

impl From<&str> for Day3 {
    fn from(value: &str) -> Self {
        Day3 {
            value: value.to_string(),
        }
    }
}

impl Represent for Day3 {
    fn get_result(&self) {
        println!("----------");
        println!("Day 3");
        println!("----------");
        println!("Part 1: {:#?}", self.part1());
        println!("Part 2: {:#?}", self.part2());
        println!("----------");
    }
}

impl Day3 {
    pub fn part1(&self) -> usize {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("no regex error");
        let mut result = 0;

        for (_, [first, second]) in re.captures_iter(self.value.as_str()).map(|c| c.extract()) {
            result += first.parse::<usize>().expect("to be parsed")
                * second.parse::<usize>().expect("to be parsed");
        }
        return result;
    }

    pub fn part2(&self) -> usize {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("no regex error");
        let mut result = 0;

        let mut can_mul = true;
        for c in re.captures_iter(self.value.as_str()) {
            if c.iter().all(|i| i.is_some()) {
                if !can_mul {
                    continue;
                }
                let first = c.get(1).unwrap().as_str();
                let second = c.get(2).unwrap().as_str();
                result += first.parse::<usize>().expect("to be parsed")
                    * second.parse::<usize>().expect("to be parsed");
            } else if let Some(m) = c.get(0) {
                match m.as_str() {
                    "do()" => can_mul = true,
                    "don't()" => can_mul = false,
                    _ => (),
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let data = include_str!("../test/day3.txt");
        let result = Day3::from(data);

        assert_eq!(result.part1(), 161);
    }

    #[test]
    fn part1_solution() {
        let data = include_str!("../data/day3.txt");
        let result = Day3::from(data);

        assert_eq!(result.part1(), 185797128);
    }

    #[test]
    fn part2() {
        let data = include_str!("../test/day3.txt");
        let result = Day3::from(data);

        assert_eq!(result.part2(), 48);
    }
}
