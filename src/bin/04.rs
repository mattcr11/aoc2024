use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    match Puzzle::try_from(input) {
        Ok(puzzle) => {
            let results = puzzle.find_word("XMAS");
            Some(results)
        }
        Err(_) => None,
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

struct Puzzle {
    content: ndarray::Array2<char>,
}

impl Puzzle {
    fn find_word(&self, input: &str) -> u32 {
        let rows = self.content.len_of(ndarray::Axis(0));
        let columns = self.content.len_of(ndarray::Axis(1));
        let input_len = input.len();
        let directions: Vec<(i32, i32)> = vec![
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        let start_locations = (0..rows).cartesian_product(0..columns);

        let target_word = input.chars().map(|c| Some(c));

        let possible_locations = start_locations
            .cartesian_product(directions.iter())
            .filter_map(|((row, column), (row_dir, column_dir))| {
                let positions = (0..input_len).map(move |offset| {
                    (
                        row as i32 + offset as i32 * row_dir,
                        column as i32 + offset as i32 * column_dir,
                    )
                });
                positions
                    .clone()
                    .all(|(row, column)| {
                        row >= 0 && row < rows as i32 && column >= 0 && column < columns as i32
                    })
                    .then_some(positions)
            });

        let filled_locations = possible_locations.map(|positions| {
            positions
                .map(|(row, column)| self.content.get((row as usize, column as usize)))
        });

        let tested_locations = filled_locations.filter_map(|positions| {
            positions.zip(target_word.clone()).all(|(a,b)| {
                a == b.as_ref()
            }).then_some(1)
        });

        tested_locations.count() as u32
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_puzzle(value) {
            Ok((_, puzzle)) => {
                let rows = puzzle.len();
                let columns = puzzle[0].len();
                let flattened_vec = puzzle.into_iter().flatten().collect_vec();
                match ndarray::Array2::from_shape_vec((rows, columns), flattened_vec) {
                    Ok(puzzle_array) => Ok(Puzzle {
                        content: puzzle_array,
                    }),
                    Err(_) => Err("Failure converting puzzle array"),
                }
            }
            Err(_) => Err("Failure parsing puzzle"),
        }
    }
}

fn parse_puzzle(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        line_ending,
        nom::combinator::map(alpha1, |row: &str| row.chars().collect_vec()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
