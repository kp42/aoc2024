use crate::Represent;

#[derive(Debug, Clone, Copy)]
enum Direction {
    West,
    East,
    North,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}
#[derive(Debug, Default)]
pub struct Day4 {
    positions: Vec<Position>,
    height: usize,
    width: usize,
}

impl Represent for Day4 {
    fn get_result(&self) {
        println!("----------");
        println!("Day 4");
        println!("----------");
        println!("Part 1: {:#?}", self.part1());
        println!("Part 2: {:#?}", self.part2());
        println!("----------");
    }
}

#[derive(Debug, Default)]
struct Position {
    x: usize,
    y: usize,
    value: String,
}

#[derive(Debug, Default)]
struct Cell {}

impl Position {
    pub fn is_next(&self, other: &Position) -> bool {
        match (self.value.as_str(), other.value.as_str()) {
            ("X", "M") | ("M", "A") | ("A", "S") => true,
            _ => false,
        }
    }

    fn is_first(&self) -> bool {
        self.value.as_str() == "X"
    }

    fn is_last(&self) -> bool {
        self.value.as_str() == "S"
    }
}

impl From<&str> for Day4 {
    fn from(value: &str) -> Self {
        let mut width: usize = 0;
        let positions: Vec<Position> = value
            .lines()
            .enumerate()
            .flat_map(|(line_index, line)| {
                width = line.len();
                return line.chars().enumerate().map(move |(char_index, char)| {
                    return Position {
                        y: char_index,
                        x: line_index,
                        value: char.to_string(),
                    };
                });
            })
            .collect();

        let height: usize = value.lines().count();

        Day4 {
            positions,
            height,
            width,
        }
    }
}

impl Day4 {
    pub fn part1(&self) -> usize {
        let starting_positions = self.get_starting_positions();
        let mut result = 0;
        for pos in starting_positions {
            if self.check_direction(pos, Direction::West) {
                result += 1;
            }
            if self.check_direction(pos, Direction::East) {
                result += 1;
            }
            if self.check_direction(pos, Direction::North) {
                result += 1;
            }
            if self.check_direction(pos, Direction::South) {
                result += 1;
            }
            if self.check_direction(pos, Direction::NorthWest) {
                result += 1;
            }
            if self.check_direction(pos, Direction::NorthEast) {
                result += 1;
            }
            if self.check_direction(pos, Direction::SouthWest) {
                result += 1;
            }
            if self.check_direction(pos, Direction::SouthEast) {
                result += 1;
            }
        }
        result
    }

    pub fn part2(&self) -> usize {
        let starting_positions = self.get_part2_starting_positions();
        let mut result = 0;
        for pos in starting_positions {
            let mut first_correct = false;
            let mut second_correct = false;
            if let Some(first_diagonal) = self.get_values_by_coords(
                (pos.x, pos.y),
                (pos.x + 1, pos.y + 1),
                (pos.x + 2, pos.y + 2),
            ) {
                let diagonal = first_diagonal
                    .iter()
                    .map(|p| p.value.clone())
                    .collect::<Vec<_>>()
                    .join("");

                if diagonal == "MAS" || diagonal == "SAM" {
                    first_correct = true;
                }
            } else {
                continue;
            }
            if let Some(second_diagonal) = self.get_values_by_coords(
                (pos.x, pos.y + 2),
                (pos.x + 1, pos.y + 1),
                (pos.x + 2, pos.y),
            ) {
                let diagonal = second_diagonal
                    .iter()
                    .map(|p| p.value.clone())
                    .collect::<Vec<_>>()
                    .join("");

                if diagonal == "MAS" || diagonal == "SAM" {
                    second_correct = true;
                }
            } else {
                continue;
            }

            if first_correct && second_correct {
                result += 1;
            }
        }
        result
    }

    fn get_starting_positions(&self) -> Vec<&Position> {
        return self.positions.iter().filter(|p| p.is_first()).collect();
    }

    fn get_part2_starting_positions(&self) -> Vec<&Position> {
        return self
            .positions
            .iter()
            .filter(|p| p.value == "M" || p.value == "S")
            .collect();
    }

    fn check_direction(&self, start_pos: &Position, direction: Direction) -> bool {
        match self.get_next_coords(start_pos, direction) {
            Some((new_x, new_y)) => {
                if let Some(next) = self.get_position_by_coords(new_x, new_y) {
                    if !start_pos.is_next(next) {
                        return false;
                    }
                    if next.is_last() {
                        return true;
                    }
                    return self.check_direction(next, direction);
                }
                false
            }
            None => false,
        }
    }

    fn get_position_by_coords(&self, x: usize, y: usize) -> Option<&Position> {
        self.positions.iter().find(|p| p.x == x && p.y == y)
    }

    fn get_next_coords(
        &self,
        start_pos: &Position,
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::West => {
                if start_pos.y > 0 {
                    return Some((start_pos.x, start_pos.y - 1));
                }
                None
            }
            Direction::East => {
                if start_pos.y < self.width {
                    return Some((start_pos.x, start_pos.y + 1));
                }
                None
            }
            Direction::North => {
                if start_pos.x > 0 {
                    return Some((start_pos.x - 1, start_pos.y));
                }
                None
            }
            Direction::South => {
                if start_pos.x < self.height {
                    return Some((start_pos.x + 1, start_pos.y));
                }
                None
            }
            Direction::NorthWest => {
                if start_pos.x > 0 && start_pos.y > 0 {
                    return Some((start_pos.x - 1, start_pos.y - 1));
                }
                None
            }
            Direction::NorthEast => {
                if start_pos.x > 0 && start_pos.y < self.width {
                    return Some((start_pos.x - 1, start_pos.y + 1));
                }
                None
            }
            Direction::SouthWest => {
                if start_pos.x < self.height && start_pos.y > 0 {
                    return Some((start_pos.x + 1, start_pos.y - 1));
                }
                None
            }
            Direction::SouthEast => {
                if start_pos.x < self.height && start_pos.y < self.width {
                    return Some((start_pos.x + 1, start_pos.y + 1));
                }
                None
            }
        }
    }

    fn get_values_by_coords(
        &self,
        first: (usize, usize),
        second: (usize, usize),
        third: (usize, usize),
    ) -> Option<Vec<&Position>> {
        let mut r: Vec<&Position> = Vec::with_capacity(3);

        if let Some(pos) = self.get_position_by_coords(first.0, first.1) {
            r.push(pos);
        }
        if let Some(pos) = self.get_position_by_coords(second.0, second.1) {
            r.push(pos);
        }
        if let Some(pos) = self.get_position_by_coords(third.0, third.1) {
            r.push(pos);
        }

        if r.len() == 3 {
            return Some(r);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let data = include_str!("../test/day4.txt");
        let result = Day4::from(data);

        assert_eq!(result.part1(), 18);
    }

    #[test]
    fn part2() {
        let data = include_str!("../test/day4.txt");
        let result = Day4::from(data);

        assert_eq!(result.part2(), 9);
    }
}
