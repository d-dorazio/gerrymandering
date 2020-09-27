mod bitfield;
pub mod board;

pub use board::Board;

use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Candidate {
    board: Board,
    boundary: HashSet<(i64, i64)>,
    current_size: i64,
}

pub fn all_possible_clusters(obstacles: &Board, size: i64) -> Vec<Board> {
    let width = obstacles.width();
    let height = obstacles.height();
    let area = obstacles.area();

    if size > area {
        return vec![];
    }

    let mut stack = Vec::with_capacity(usize::try_from(area).unwrap_or(0));
    stack.push(Candidate {
        board: Board::new(width, height),
        current_size: 0,
        boundary: (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .filter(|&(xx, yy)| !obstacles.get(xx, yy))
            .collect(),
    });

    let mut seen = HashSet::new();
    let mut result = Vec::new();

    while let Some(Candidate {
        board,
        boundary,
        current_size,
    }) = stack.pop()
    {
        if current_size == size {
            result.push(board);
            continue;
        }

        for &(x, y) in &boundary {
            let mut new_board = board.clone();
            new_board.set(x, y);

            if !seen.insert(new_board.data.data()) {
                continue;
            }

            let mut new_boundary = HashSet::new();
            for yy in 0..height {
                for xx in 0..width {
                    if new_board.get(xx, yy) || obstacles.get(xx, yy) {
                        continue;
                    }

                    let neighbors = [(xx + 1, yy), (xx - 1, yy), (xx, yy + 1), (xx, yy - 1)];
                    let mut neighbors = neighbors
                        .iter()
                        .filter(|&&(xx, yy)| xx >= 0 && xx < width && yy >= 0 && yy < height);

                    let touches_cluster =
                        neighbors.any(|&(xx, yy)| new_board.get(xx, yy) && !obstacles.get(xx, yy));

                    if !touches_cluster {
                        continue;
                    }

                    new_boundary.insert((xx, yy));
                }
            }

            stack.push(Candidate {
                board: new_board,
                boundary: new_boundary,
                current_size: current_size + 1,
            })
        }
    }

    if cfg!(debug_assertions) {
        use std::collections::BTreeSet;

        let mut all = result.iter().map(|b| b.data.data()).collect::<Vec<_>>();
        all.sort();

        let uniq = result
            .iter()
            .map(|b| b.data.data())
            .collect::<BTreeSet<_>>();

        assert_eq!(all.len(), uniq.len());
        assert!(all.into_iter().eq(uniq));
    }

    result
}
