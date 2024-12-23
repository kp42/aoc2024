use crate::Represent;

#[derive(Debug, Default)]
pub struct Day1 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Represent for Day1 {
    fn get_result(&self) {
        println!("----------");
        println!("Day 1");
        println!("----------");
        println!("Part 1: {:#?}", self.part1());
        println!("Part 2: {:#?}", self.part2());
        println!("----------");
    }
}

impl From<&str> for Day1 {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let mut r = Day1::default();

        for line in lines {
            let mut is_left = true;
            for v in line.split_whitespace() {
                if is_left {
                    r.left.push(v.parse().expect("to be parsed"));
                    is_left = false;
                } else {
                    r.right.push(v.parse().expect("to be parsed"));
                }
            }
        }

        r.left.sort();
        r.right.sort();

        r
    }
}

impl Day1 {
    pub fn part1(&self) -> usize {
        let mut result = 0;

        for (l, r) in self.left.iter().zip(self.right.iter()) {
            result += &l.abs_diff(*r);
        }

        return result;
    }

    pub fn part2(&self) -> usize {
        let mut result = 0;
        for l in &self.left {
            let similarity_score = self.right.iter().filter(|r| l.eq(r)).count();
            result += similarity_score * l;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let data = include_str!("../test/day1.txt");
        let result = Day1::from(data);

        assert_eq!(result.part1(), 11);
    }

    #[test]
    fn part2() {
        let data = include_str!("../test/day1.txt");
        let result = Day1::from(data);

        assert_eq!(result.part2(), 31);
    }
}
