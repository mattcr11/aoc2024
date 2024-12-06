use nom::{bytes::complete::tag, combinator::value, IResult};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    match multiple_mul_parser(input) {
        Ok((_, muls)) => Some(
            muls.iter()
                .filter_map(|op| match op {
                    Operation::DO => None,
                    Operation::DONT => None,
                    Operation::MUL(first, second) => Some(first * second),
                })
                .sum::<i32>() as u32,
        ),
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match multple_op_parser(input) {
        Ok((_, ops)) => {
            let mut enabled = true;
            let count = ops
                .iter()
                .filter_map(|op| match op {
                    Operation::DO => {
                        enabled = true;
                        None
                    }
                    Operation::DONT => {
                        enabled = false;
                        None
                    }
                    Operation::MUL(first, second) => enabled.then_some(first * second),
                })
                .sum::<i32>() as u32;
            Some(count)
        }
        Err(_) => None,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Operation {
    DO,
    DONT,
    MUL(i32, i32),
}

fn multple_op_parser(mut input: &str) -> IResult<&str, Vec<Operation>> {
    let mut output = Vec::new();
    while !input.is_empty() {
        match op_parser(input) {
            Ok((remaining, new_op)) => {
                output.push(new_op);
                input = remaining;
            }
            Err(_) => {
                input = &input[1..];
            }
        }
    }
    Ok(("", output))
}

fn op_parser(input: &str) -> IResult<&str, Operation> {
    nom::branch::alt((mul_parser, do_parser, dont_parser))(input)
}

fn do_parser(input: &str) -> IResult<&str, Operation> {
    value(Operation::DO, tag("do()"))(input)
}

fn dont_parser(input: &str) -> IResult<&str, Operation> {
    value(Operation::DONT, tag("don't()"))(input)
}

fn multiple_mul_parser(mut input: &str) -> IResult<&str, Vec<Operation>> {
    let mut output = Vec::new();
    while !input.is_empty() {
        match mul_parser(input) {
            Ok((remaining, new_mul)) => {
                output.push(new_mul);
                input = remaining;
            }
            Err(_) => input = &input[1..],
        }
    }
    Ok(("", output))
}

fn mul_parser(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (first_arg, second_arg)) = nom::sequence::separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Operation::MUL(first_arg, second_arg)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_parser() {
        let (_, result) = mul_parser("mul(123,231)").unwrap();
        assert_eq!(result, Operation::MUL(123, 231));
    }

    #[test]
    fn test_multiple_mul_parser() {
        let (_, result) =
            multiple_mul_parser(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(
            result,
            vec!(
                Operation::MUL(2, 4),
                Operation::MUL(5, 5),
                Operation::MUL(11, 8),
                Operation::MUL(8, 5)
            )
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
