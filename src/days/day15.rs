use crate::common::Solution;

fn hash(s: &str) -> u8 {
    s.chars().fold(0, |digest, c| {
        (digest.wrapping_add(c as u8)).wrapping_mul(17)
    })
}

fn solve_a(steps: &[&str]) -> u32 {
    steps.iter().map(|step| u32::from(hash(step))).sum()
}

fn solve_b(steps: &[&str]) -> usize {
    steps
        .iter()
        .fold(
            std::array::from_fn(Vec::with_capacity),
            |mut map: [Vec<(&str, u8)>; 256], step| {
                let (label, add_flen): (&str, Option<u8>) =
                    if let Some((label, flen)) = step.split_once('=') {
                        (label, Some(flen.parse().unwrap()))
                    } else if let Some(label) = step.strip_suffix('-') {
                        (label, None)
                    } else {
                        unimplemented!()
                    };
                let hash = hash(label) as usize;
                if let Some(flen) = add_flen {
                    if let Some((_, existing_flen)) =
                        map[hash].iter_mut().find(|(l, _)| *l == label)
                    {
                        *existing_flen = flen;
                    } else {
                        map[hash].push((label, flen));
                    }
                } else {
                    map[hash].retain(|(l, _)| *l != label);
                }
                map
            },
        )
        .iter()
        .enumerate()
        .flat_map(|(i_box, lenses)| {
            let box_num = i_box + 1;
            lenses
                .iter()
                .enumerate()
                .map(move |(i_lens, (_, flen))| box_num * (i_lens + 1) * (*flen as usize))
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let steps: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(','))
        .collect();

    (solve_a(&steps).to_string(), solve_b(&steps).to_string())
}
