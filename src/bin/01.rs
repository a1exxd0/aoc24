advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut vec1, mut vec2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut vec1, mut vec2), (a, b)| {
                vec1.push(a);
                vec2.push(b);
                (vec1, vec2)
            }
        );

    vec1.sort(); vec2.sort();

    Some (
        vec1.iter().zip(vec2.iter()).map(|(a, b)| (*a as i32 - *b as i32)).sum::<i32>() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (vec, occurences): (Vec<u32>, HashMap<u32, u32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut vec, mut occurences), (a, b)| {
                vec.push(a);
                *occurences.entry(b).or_default() += 1;
                (vec, occurences)
            },
        );

    let result = vec
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
