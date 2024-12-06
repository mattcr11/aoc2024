use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    solve_part_one(input).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_part_two(input).ok()
}

fn solve_part_one(input: &str) -> anyhow::Result<u32> {
    let reports = parse_input(input)?;
    Ok(reports
        .iter()
        .filter_map(|item| is_report_safe_part1(item).then_some(()))
        .count() as u32)
}

fn solve_part_two(input: &str) -> anyhow::Result<u32> {
    let reports = parse_input(input)?;
    Ok(reports
        .iter()
        .filter_map(|item| is_report_safe_part2(item).then_some(()))
        .count() as u32)
}

fn analyze_report_differences(report: &Vec<i32>) -> Vec<i32> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(first, second)| first - second)
        .collect_vec()
}

fn find_first_fault(analysis: &Vec<i32>) -> Option<usize> {
    let first_non_zero = analysis.iter().find(|item| **item != 0);
    match first_non_zero {
        Some(first_non_zero) => {
            let starting_sign = first_non_zero.signum();
            analysis
                .iter()
                .find_position(|item| {
                    **item == 0 || item.signum() != starting_sign || item.abs() > 3
                })
                .map(|(pos, _)| pos)
        }
        None => Some(0),
    }
}

fn is_report_safe_part1(report: &Vec<i32>) -> bool {
    find_first_fault(&analyze_report_differences(report)).is_none()
}

fn is_report_safe_part2(report: &Vec<i32>) -> bool {
    match find_first_fault(&analyze_report_differences(report)) {
        Some(fault_position) => (fault_position.saturating_sub(1)
            ..report.len().min(fault_position + 2)) // check positions on both sides of fault
            .map(|report_position| {
                let mut tolerated_report = report.clone();
                let _ = tolerated_report.remove(report_position as usize);
                is_report_safe_part1(&tolerated_report)
            })
            .any(|safe| safe),
        None => true,
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let rows = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_safe_part1() {
        let reports = parse_input(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        let safe_result = is_report_safe_part1(&reports[0]);
        assert!(safe_result);

        let unsafe_result = is_report_safe_part1(&reports[1]);
        assert!(!unsafe_result);
    }

    #[test]
    fn test_report_safe_part2() {
        let report = vec![31, 32, 30, 33, 34, 37];
        let result = is_report_safe_part2(&report);
        assert!(result);
    }

    #[test]
    fn test_find_fault() {
        let result = find_first_fault(&vec![1, 2, 7, 8, 9]);
        assert_eq!(result, Some(2))
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
