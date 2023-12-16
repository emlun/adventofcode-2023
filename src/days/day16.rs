use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

enum Tile {
    MirrorNE,
    MirrorSE,
    SplitterNS,
    SplitterWE,
}

type BeamPos = ((usize, usize), (isize, isize));

fn solve_a(map: &HashMap<(usize, usize), Tile>, max_r: usize, max_c: usize) -> usize {
    let mut beams: Vec<BeamPos> = vec![((0, 0), (0, 1))];
    let mut energized: HashSet<BeamPos> = beams.iter().copied().collect();
    let energized = loop {
        beams = beams
            .into_iter()
            .flat_map(|((r, c), (dr, dc))| {
                r.checked_add_signed(dr)
                    .and_then(|rr| c.checked_add_signed(dc).map(|cc| (rr, cc)))
                    .into_iter()
                    .flat_map(move |(rr, cc)| {
                        use Tile::*;
                        (match ((dr, dc), map.get(&(rr, cc))) {
                            ((0, 1), Some(MirrorNE)) => Some([(-1, 0)].iter()),
                            ((0, -1), Some(MirrorNE)) => Some([(1, 0)].iter()),
                            ((1, 0), Some(MirrorNE)) => Some([(0, -1)].iter()),
                            ((-1, 0), Some(MirrorNE)) => Some([(0, 1)].iter()),
                            ((0, 1), Some(MirrorSE)) => Some([(1, 0)].iter()),
                            ((0, -1), Some(MirrorSE)) => Some([(-1, 0)].iter()),
                            ((1, 0), Some(MirrorSE)) => Some([(0, 1)].iter()),
                            ((-1, 0), Some(MirrorSE)) => Some([(0, -1)].iter()),
                            ((0, _), Some(SplitterNS)) => Some([(1, 0), (-1, 0)].iter()),
                            ((_, 0), Some(SplitterWE)) => Some([(0, 1), (0, -1)].iter()),
                            _ => None,
                        })
                        .unwrap_or([(dr, dc)].iter())
                        .map(move |(drr, dcc)| ((rr, cc), (*drr, *dcc)))
                        .collect::<Vec<_>>()
                    })
            })
            .filter(|((r, c), _)| *r <= max_r && *c <= max_c)
            .filter(|beam_pos| !energized.contains(beam_pos))
            .collect();
        let elen = energized.len();
        energized.extend(&beams);
        if elen == energized.len() {
            break energized;
        }
    };
    energized
        .into_iter()
        .map(|((r, c), _)| (r, c))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

pub fn solve(lines: &[String]) -> Solution {
    let (map, max_r, max_c): (HashMap<(usize, usize), Tile>, usize, usize) = lines
        .iter()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().flat_map(move |(c, chr)| {
                (match chr {
                    '/' => Some(Tile::MirrorNE),
                    '\\' => Some(Tile::MirrorSE),
                    '|' => Some(Tile::SplitterNS),
                    '-' => Some(Tile::SplitterWE),
                    _ => None,
                })
                .map(|tile| ((r, c), tile))
            })
        })
        .fold(
            (HashMap::new(), 0, 0),
            |(mut map, max_r, max_c), ((r, c), tile)| {
                map.insert((r, c), tile);
                (map, std::cmp::max(max_r, r), std::cmp::max(max_c, c))
            },
        );

    (solve_a(&map, max_r, max_c).to_string(), "".to_string())
}
