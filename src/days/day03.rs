use std::collections::{HashMap, HashSet};

use crate::common::Solution;

fn solve_a(lines: &[String]) -> u32 {
    let mut parts: HashSet<(usize, usize)> = HashSet::new();
    let mut nums: HashMap<(usize, usize, isize), u32> = HashMap::new();

    for (r, line) in lines.iter().enumerate() {
        let mut current_num = None;
        for (c, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                if let Some(((r, c, l), cur)) = current_num {
                    current_num = Some((
                        (r, c, l + 1),
                        cur * 10 + chr.to_string().parse::<u32>().unwrap(),
                    ));
                } else {
                    current_num = Some(((r, c, 1), chr.to_string().parse().unwrap()));
                }
            } else {
                if let Some((pos, num)) = current_num {
                    nums.insert(pos, num);
                    current_num = None;
                }
                if chr != '.' {
                    parts.insert((r, c));
                }
            }
        }
        if let Some((pos, num)) = current_num {
            nums.insert(pos, num);
            current_num = None;
        }
    }

    let (part_numbers, non_part_numbers): (
        HashMap<(usize, usize, isize), u32>,
        HashMap<(usize, usize, isize), u32>,
    ) = nums.into_iter().partition(|((numr, numc, numl), _)| {
        for (dr, dc) in (-1..=1).flat_map(|r| (-1..=*numl).map(move |c| (r, c))) {
            if parts.contains(&(
                numr.saturating_add_signed(dr),
                numc.saturating_add_signed(dc),
            )) {
                return true;
            }
        }
        return false;
    });

    dbg!(part_numbers.len(), non_part_numbers.len());

    part_numbers.into_iter().map(|(_, num)| num).sum()
}

fn solve_b() -> u32 {
    todo!()
}

pub fn solve(lines: &[String]) -> Solution {
    (solve_a(lines).to_string(), "".to_string())
}
