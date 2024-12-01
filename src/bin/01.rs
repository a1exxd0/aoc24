advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    vec1.sort(); vec2.sort();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs() as u32)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (vec1, vec2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let occurences: HashMap<u32, u32> = vec2
        .iter()
        .fold(HashMap::new(), |mut acc, &n| {
            *acc.entry(n).or_default() += 1;
            acc
        });

    let result = vec1
        .iter()
        .map(|&n| n * occurences.get(&n).copied().unwrap_or(0))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, result);
    }
}
