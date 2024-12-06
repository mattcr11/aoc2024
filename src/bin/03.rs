use nom::{bytes::complete::tag, IResult};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    match multiple_mul_parser(input) {
        Ok((_, muls)) => Some(
            muls.iter()
                .map(|(first, second)| first * second)
                .sum::<i32>() as u32,
        ),
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn multiple_mul_parser(mut input: &str) -> IResult<&str, Vec<(i32, i32)>> {
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

fn mul_parser(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (first_arg, second_arg)) = nom::sequence::separated_pair(
        nom::character::complete::i32,
        tag(","),
        nom::character::complete::i32,
    )(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (first_arg, second_arg)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_parser() {
        let (_, result) = mul_parser("mul(123,231)").unwrap();
        assert_eq!(result, (123, 231));
    }

    #[test]
    fn test_multiple_mul_parser() {
        let (_, result) =
            multiple_mul_parser(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, vec!((2, 4), (5, 5), (11, 8), (8, 5)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
