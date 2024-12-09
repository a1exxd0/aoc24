advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

fn check_bounds(row_c: usize, col_c: usize, r: i32, c: i32) -> bool {
    !(c < 0 || r < 0 || r as usize >= row_c || c as usize >= col_c)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut result_set: HashSet<(i32, i32)> = HashSet::new();
    let (row_c, col_c) = input
        .lines()
        .enumerate()
        .fold((0, 0), |(max_row, max_col), (i, line)| {
            line.chars().enumerate().for_each(|(j, chr)| {
                if chr != '.' {
                    map.entry(chr)
                        .or_insert_with(Vec::new)
                        .push((i as i32, j as i32));
                }
            });
            (i + 1, line.len().max(max_col))
        });

    map.values().for_each(|antennas| {
        antennas.iter().for_each(|&a1| {
            antennas.iter().filter(|&&a2| a1 != a2).for_each(|&a2| {
                let (new_x, new_y) = (
                    a2.0 + (a2.0 - a1.0), a2.1 + (a2.1 - a1.1)
                );
                if new_x >= 0 && new_x < row_c as i32 && new_y >= 0 && new_y < col_c as i32 {
                    result_set.insert((new_x, new_y));
                }
            });
        });
    });

    Some(result_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut result_set: HashSet<(i32, i32)> = HashSet::new();
    let (mut row_c, mut col_c) = (0, 0);

    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(j, chr)| {
                    if chr != '.' {
                        map
                        .entry(chr)
                        .or_insert_with(Vec::new)
                        .push((i as i32, j as i32));
                    }
                    col_c = j + 1;
                });
            row_c = i + 1;
        });

    for key in map.keys() {
        let antennas = map.get(key).unwrap();
        for &a1 in antennas {
            for &a2 in antennas {
                if a1 == a2 { continue; }

                let (dx, dy) = ((a2.0 - a1.0), (a2.1 - a1.1));

                for i in 0.. {
                    let (new_x, new_y) = (a2.0 + dx * i, a2.1 + dy * i);
                    if check_bounds(row_c, col_c, new_x, new_y) {
                        result_set.insert((new_x, new_y));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Some(result_set.len() as u32)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
