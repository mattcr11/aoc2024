advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // convert string into vector of vectors of numbers in the same configuration
    let pairs = input
        .lines()
        .map(|line| {
            line.split("   ")
                .map(|item| item.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // split the lists into individual vectors and sort
    let sorted = (0..pairs[0].len())
        .map(|list_index| {
            let mut unsorted = pairs
                .iter()
                .map(|line| line[list_index])
                .collect::<Vec<_>>();
            unsorted.sort();
            unsorted
        })
        .collect::<Vec<_>>();

    // compute pair-wise distances
    let distances = (0..pairs.len()).map(|line_index| {
        (0..sorted.len()).map(|list_index| {
            sorted[list_index][line_index]
        }).reduce(|acc, e| acc.abs_diff(e)).unwrap()
    }).collect::<Vec<_>>();

    // sum the distances
    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
