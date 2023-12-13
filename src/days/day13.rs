use crate::common::Solution;

fn solve_a(patterns: &[Vec<Vec<bool>>]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let horiz = (1..(pattern[0].len()))
                .find(|reflection_col| {
                    pattern.iter().all(|row| {
                        std::iter::zip(
                            row.iter().take(*reflection_col).rev(),
                            row.iter().skip(*reflection_col),
                        )
                        .all(|(a, b)| a == b)
                    })
                })
                .unwrap_or(0);

            let vert = (1..(pattern.len()))
                .find(|reflection_row| {
                    std::iter::zip(
                        pattern.iter().take(*reflection_row).rev(),
                        pattern.iter().skip(*reflection_row),
                    )
                    .all(|(a, b)| a == b)
                })
                .unwrap_or(0);

            assert!(vert == 0 || horiz == 0);
            100 * vert + horiz
        })
        .sum()
}

fn solve_b(patterns: &[Vec<Vec<bool>>]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let horiz = (1..(pattern[0].len()))
                .find(|reflection_col| {
                    pattern
                        .iter()
                        .map(|row| {
                            std::iter::zip(
                                row.iter().take(*reflection_col).rev(),
                                row.iter().skip(*reflection_col),
                            )
                            .filter(|(a, b)| a != b)
                            .count()
                        })
                        .sum::<usize>()
                        == 1
                })
                .unwrap_or(0);

            let vert = (1..(pattern.len()))
                .find(|reflection_row| {
                    std::iter::zip(
                        pattern.iter().take(*reflection_row).rev(),
                        pattern.iter().skip(*reflection_row),
                    )
                    .map(|(row_a, row_b)| row_a.iter().zip(row_b).filter(|(a, b)| a != b).count())
                    .sum::<usize>()
                        == 1
                })
                .unwrap_or(0);

            assert!(vert == 0 || horiz == 0);
            100 * vert + horiz
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (mut patterns, last_pattern): (Vec<Vec<Vec<bool>>>, _) =
        lines.iter().map(|s| s.trim()).fold(
            (Vec::new(), Vec::new()),
            |(mut patterns, mut last_pattern), line| {
                if line.is_empty() {
                    if !last_pattern.is_empty() {
                        patterns.push(last_pattern);
                        last_pattern = Vec::new();
                    }
                } else {
                    last_pattern.push(line.chars().map(|chr| chr == '#').collect());
                }
                (patterns, last_pattern)
            },
        );
    if !last_pattern.is_empty() {
        patterns.push(last_pattern);
    }

    (
        solve_a(&patterns).to_string(),
        solve_b(&patterns).to_string(),
    )
}
