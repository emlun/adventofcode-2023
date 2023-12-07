use crate::common::Solution;

/// Solve the equation x^2 + px + q = 0, assuming real solutions exist.
fn solve_quadratic(p: f64, q: f64) -> (f64, f64) {
    let p_half = p / 2.0;
    let sqrt_part = ((p / 2.0).powi(2) - q).sqrt();
    (-p_half - sqrt_part, -p_half + sqrt_part)
}

#[cfg(test)]
#[test]
fn test_solve_quadratic() {
    assert_eq!((0.0, 2.0), solve_quadratic(-2.0, 0.0));
    let (x1, x2) = solve_quadratic(0.0, 1.0);
    assert!(x1.is_nan());
    assert!(x2.is_nan());
}

fn solve_a(races: &[(u64, u64)]) -> usize {
    races
        .iter()
        .map(|(time, distance)| {
            let (t1, t2) = solve_quadratic(-(*time as f64), *distance as f64);
            (t2.floor() - t1.ceil()) as usize + 1
        })
        .product()
}

fn solve_b(races: &[(u64, u64)]) -> usize {
    let (time, distance) = races
        .iter()
        .fold((0_u64, 0_u64), |(time, distance), (t, d)| {
            (
                time * u64::from(10_u32.pow(t.ilog10() + 1)) + *t,
                distance * u64::from(10_u32.pow(d.ilog10() + 1)) + *d,
            )
        });
    let (t1, t2) = solve_quadratic(-(time as f64), distance as f64);
    (t2.floor() - t1.ceil()) as usize + 1
}

pub fn solve(lines: &[String]) -> Solution {
    let times: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.parse().ok())
        .collect();
    let races: Vec<(u64, u64)> = times.into_iter().zip(distances).collect();
    (solve_a(&races).to_string(), solve_b(&races).to_string())
}
