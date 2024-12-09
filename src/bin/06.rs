advent_of_code::solution!(6);
use std::{collections::HashSet, thread::sleep, time::Duration};

fn bounds_check(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    !(x < 0 || y < 0 || x as usize >= map.len() || y as usize >= map[0].len())
}

fn get_res(map: &mut Vec<Vec<char>>, x: i32, y: i32, new_x: i32, new_y: i32, next_chr: char) -> Option<(i32, i32)> {
    if !bounds_check(map, new_x, new_y) {
        map[x as usize][y as usize] = 'X';
        None
    } else if map[new_x as usize][new_y as usize] != '#' {
        map[new_x as usize][new_y as usize] = map[x as usize][y as usize];
        map[x as usize][y as usize] = 'X';
        Some((new_x, new_y))
    } else {
        map[x as usize][y as usize] = next_chr;
        Some((x, y))
    }
}

fn static_get_res(map: &mut Vec<Vec<char>>, x: i32, y: i32, new_x: i32, new_y: i32, next_chr: char) -> Option<(i32, i32)> {
    if !bounds_check(map, new_x, new_y) {
        None
    } else if map[new_x as usize][new_y as usize] != '#' {
        map[new_x as usize][new_y as usize] = map[x as usize][y as usize];
        Some((new_x, new_y))
    } else {
        map[x as usize][y as usize] = next_chr;
        Some((x, y))
    }
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                return (i, j);
            }
         }
    }

    (0, 0)
}

pub fn transform(map: &mut Vec<Vec<char>>, x: i32, y: i32) -> Option<(i32, i32)> {
    if !bounds_check(map, x, y) {
        return None;
    }

    match map[x as usize][y as usize] {
        '^' => {
            let (new_x, new_y, next_chr) = (x - 1, y, '>');
            get_res(map, x, y, new_x, new_y, next_chr)
        },
        '>' => {
            let (new_x, new_y, next_chr) = (x, y + 1, 'D');
            get_res(map, x, y, new_x, new_y, next_chr)
        },
        'D' => {
            let (new_x, new_y, next_chr) = (x + 1, y, '<');
            get_res(map, x, y, new_x, new_y, next_chr)
        },
        '<' => {
            let (new_x, new_y, next_chr) = (x, y - 1, '^');
            get_res(map, x, y, new_x, new_y, next_chr)
        }
        _ => panic!("found unexpected"),
    }
}

fn static_transform(map: &mut Vec<Vec<char>>, x: i32, y: i32) -> Option<(i32, i32)> {
    if !bounds_check(map, x, y) {
        return None;
    }

    match map[x as usize][y as usize] {
        '^' => {
            let (new_x, new_y, next_chr) = (x - 1, y, '>');
            static_get_res(map, x, y, new_x, new_y, next_chr)
        },
        '>' => {
            let (new_x, new_y, next_chr) = (x, y + 1, 'D');
            static_get_res(map, x, y, new_x, new_y, next_chr)
        },
        'D' => {
            let (new_x, new_y, next_chr) = (x + 1, y, '<');
            static_get_res(map, x, y, new_x, new_y, next_chr)
        },
        '<' => {
            let (new_x, new_y, next_chr) = (x, y - 1, '^');
            static_get_res(map, x, y, new_x, new_y, next_chr)
        }
        _ => panic!("found unexpected"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("hello");
    let mut map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();


    let (mut x, mut y) = find_start(&map);

    loop {
        let res = transform(&mut map, x as i32, y as i32);
        if res.is_none() {
            break;
        } else if let Some((a, b)) = res {
            (x, y) = (a as usize, b as usize);
        }

        // for row in &map {
        //     println!("{:?}", row.iter().collect::<String>());
        // }
        // println!("\n");
        // sleep(Duration::from_secs(1));
    }

    Some(map
        .iter()
        .flat_map(|vec| vec.iter())
        .filter(|&&elem| elem == 'X')
        .count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let _map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (init_x, init_y) = find_start(&_map);
    let mut fin: u32 = 0;

    for i in 0.._map.len() {
        for j in 0.._map[0].len() {
            if _map[i][j] != '.' {
                continue;
            }

            let mut map = _map.clone();
            let (mut x, mut y) = (init_x, init_y);
            let mut turns = HashSet::<(usize, usize, char)>::new();

            map[i][j] = '#';
            loop {
                let res = static_transform(&mut map, x as i32, y as i32);

                if res.is_none() {
                    break;
                } else if let Some((a, b)) = res {
                    if turns.contains(&(a as usize, b as usize, map[x][y])) {
                        //println!("valid: {}, {}, {}", a, b, map[x][y]);
                        fin += 1;
                        break;
                    } else if (x as i32, y as i32) == (a, b) {
                        //println!("insert: {}, {}, {}", a, b, map[x][y]);
                        turns.insert((a as usize, b as usize, map[x][y]));
                        (x, y) = (a as usize, b as usize);
                    } else {
                        (x, y) = (a as usize, b as usize);
                    }
                }
            }
        }

        println!("final, ind: {}, {}", fin, i);
    }

    fin.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#.trim_start();
        let result: Option<u32> = part_one(input);
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let input: &str = r#"
....#.....
.........#,
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#.trim_start();

        let result = part_two(input);
        assert_eq!(result, Some(6));
    }
}
