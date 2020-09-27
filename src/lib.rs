mod bitfield;
pub mod board;

use bitfield::Bitfield;
pub use board::Board;

use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct Candidate {
    board: Board,
    boundary: Bitfield,
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
        boundary: {
            let mut bf = Bitfield::new();
            for y in 0..height {
                for x in 0..width {
                    if !obstacles.get(x, y) {
                        bf.set(y * width + x);
                    }
                }
            }
            bf
        },
    });

    // TODO: seen could be a dynamic bitfield in case it's needed
    let mut seen = vec![false; 1 << area];
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

        for i in boundary.ones() {
            let x = i % width;
            let y = i / width;

            let mut new_board = board.clone();
            new_board.set(x, y);

            let seen_ix = usize::try_from(new_board.data.data()).unwrap();
            if seen[seen_ix] {
                continue;
            }
            seen[seen_ix] = true;

            let mut new_boundary = Bitfield::new();
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

                    new_boundary.set(yy * width + xx);
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
