use crate::common::Solution;

fn expand(map: &[Vec<bool>], size: usize) -> Vec<(usize, usize)> {
    let size = size - 1;
    let expand_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|b| !*b))
        .map(|(r, _)| r)
        .collect();
    let expand_cols: Vec<usize> = (0..map[0].len())
        .filter(|c| map.iter().all(|row| !row[*c]))
        .collect();

    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .map(move |(c, _)| (c, r))
        })
        .map(|(c, r)| {
            let col_expansion_factor = expand_cols.iter().take_while(|cc| **cc < c).count();
            let row_expansion_factor = expand_rows.iter().take_while(|rr| **rr < r).count();
            (
                c + col_expansion_factor * size,
                r + row_expansion_factor * size,
            )
        })
        .collect()
}

fn solve_ab(map: &[Vec<bool>], size: usize) -> usize {
    let map = expand(map, size);
    map.iter()
        .enumerate()
        .flat_map(|(i, pos1)| {
            map.iter().skip(i + 1).map(|pos2| {
                let (x1, y1) = pos1;
                let (x2, y2) = pos2;
                x1.abs_diff(*x2) + y1.abs_diff(*y2)
            })
        })
        .sum::<usize>()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<bool>> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|chr| chr == '#').collect())
        .collect();

    (
        solve_ab(&map, 2).to_string(),
        solve_ab(&map, 1_000_000).to_string(),
    )
}
