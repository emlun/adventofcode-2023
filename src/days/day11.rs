use crate::common::Solution;
use crate::util::iter::WithSliding;

fn solve_dim(v: &[i64], expansion_size: i64) -> i64 {
    let expansion_insertion_size = expansion_size - 1;
    let expansions: Vec<i64> = [0]
        .into_iter()
        .chain(v.iter().sliding2().scan(0, |exp, (a, b)| {
            *exp += std::cmp::max(0, b - a - 1) * expansion_insertion_size;
            Some(*exp)
        }))
        .collect();
    v.iter()
        .enumerate()
        .zip(expansions)
        .map(|((i, x), exp)| (2 * i as i64 - v.len() as i64 + 1) * (x + exp))
        .sum()
}

fn solve_ab(xs: &[i64], ys: &[i64], expansion_size: i64) -> i64 {
    solve_dim(xs, expansion_size) + solve_dim(ys, expansion_size)
}

pub fn solve(lines: &[String]) -> Solution {
    let (mut xs, ys): (Vec<i64>, Vec<i64>) = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold((Vec::new(), Vec::new()), |(mut xs, mut ys), (r, line)| {
            ys.resize(
                ys.len() + line.chars().filter(|c| *c == '#').count(),
                r as i64,
            );
            xs.extend(
                line.chars()
                    .enumerate()
                    .filter(|(_, chr)| *chr == '#')
                    .map(|(c, _)| c as i64),
            );
            (xs, ys)
        });
    xs.sort();

    (
        solve_ab(&xs, &ys, 2).to_string(),
        solve_ab(&xs, &ys, 1_000_000).to_string(),
    )
}
