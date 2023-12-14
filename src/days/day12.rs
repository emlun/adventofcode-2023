use crate::common::Solution;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Oper,
    Brok,
    Unkn,
}

fn matches(stati: &[Status], pattern: &[Status]) -> bool {
    stati
        .iter()
        .zip(pattern)
        .all(|(s, p)| *p == Status::Unkn || s == p)
}

fn prefix_matches(broken_counts: &[usize], extra_ok_counts: &[usize], pattern: &[Status]) -> bool {
    use Status::*;
    let mut prefix = Vec::with_capacity(pattern.len());
    prefix.resize(extra_ok_counts[0], Oper);
    for (broken, ok) in broken_counts.iter().zip(extra_ok_counts[1..].iter()) {
        prefix.resize(prefix.len() + broken, Brok);
        if prefix.len() < pattern.len() {
            prefix.push(Oper);
        }
        prefix.resize(prefix.len() + ok, Oper);
    }
    matches(&prefix, pattern)
}

fn count_combinations(
    broken_counts: &[usize],
    extra_ok_counts: &[usize],
    pattern: &[Status],
) -> usize {
    let ok_to_insert: usize = pattern.len()
        - (broken_counts.iter().sum::<usize>() + broken_counts.len() - 1)
        - extra_ok_counts.iter().sum::<usize>();
    let min_ok_to_insert = if extra_ok_counts.len() == broken_counts.len() {
        ok_to_insert
    } else {
        0
    };
    (min_ok_to_insert..=ok_to_insert)
        .map(|next_ok| {
            let ok_counts: Vec<usize> = extra_ok_counts.iter().copied().chain([next_ok]).collect();
            if prefix_matches(broken_counts, &ok_counts, pattern) {
                if ok_counts.len() == broken_counts.len() + 1 {
                    1
                } else {
                    count_combinations(broken_counts, &ok_counts, pattern)
                }
            } else {
                0
            }
        })
        .sum()
}

fn solve_a(springs: &[(Vec<Status>, Vec<usize>)]) -> usize {
    springs
        .iter()
        .map(|(pattern, broken_counts)| count_combinations(broken_counts, &[], pattern))
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let springs: Vec<(Vec<Status>, Vec<usize>)> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(|chr| match chr {
                        '.' => Status::Oper,
                        '#' => Status::Brok,
                        '?' => Status::Unkn,
                        _ => unimplemented!(),
                    })
                    .collect(),
                parts
                    .next()
                    .unwrap()
                    .split(',')
                    .flat_map(|s| s.parse().ok())
                    .collect(),
            )
        })
        .collect();

    (solve_a(&springs).to_string(), "".to_string())
}
