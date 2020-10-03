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

            let seen_ix = usize::try_from(new_board.board_id()).unwrap();
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
            });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use proptest::prelude::*;

    fn arb_board() -> BoxedStrategy<Board> {
        (1..=3_i64, 1..=3_i64)
            .prop_map(|(w, h)| Board::new(w, h))
            .boxed()
    }

    prop_compose! {
        fn arb_board_with_size()(b in arb_board())
                                (size in 1..=b.area(), b in Just(b))
                                -> (Board, i64)
        {
            (b, size)
        }
    }

    proptest! {
        #[test]
        fn test_all_possible_clusters_of_0(board in arb_board()) {
            let clusters = all_possible_clusters(&board, 0);

            assert_eq!(clusters.len(), 1);
        }

        #[test]
        fn test_all_possible_clusters_of_1(board in arb_board()) {
            let clusters = all_possible_clusters(&board, 1);

            let len = usize::try_from(board.width()).unwrap() * usize::try_from(board.height()).unwrap();
            assert_eq!(clusters.len(), len);
        }

        #[test]
        fn test_all_possible_clusters_of_size(board in arb_board()) {
            let clusters = all_possible_clusters(&board, board.area());

            assert_eq!(clusters.len(), 1);
        }

        #[test]
        fn test_no_duplicate_clusters((board, size) in arb_board_with_size()) {
            let clusters = all_possible_clusters(&board, size);

            let mut all = clusters.iter().map(|b| b.board_id()).collect::<Vec<_>>();
            all.sort();

            let uniq = clusters
                .iter()
                .map(|b| b.board_id())
                .collect::<BTreeSet<_>>();

            assert_eq!(all.len(), uniq.len());
            assert!(all.into_iter().eq(uniq));
        }
    }

    fn assert_clusters_eq(clusters: &[Board], expected_ids: &[u64]) {
        assert_eq!(clusters.len(), expected_ids.len());

        let mut all = clusters.iter().map(|b| b.board_id()).collect::<Vec<_>>();
        all.sort();

        assert_eq!(all, expected_ids);
    }

    #[test]
    fn test_all_possible_clusters_of_2x2() {
        let board = Board::new(2, 2);

        {
            let clusters = all_possible_clusters(&board, 2);
            assert_clusters_eq(&clusters, &[0x03, 0x05, 0x0A, 0x0C]);
        }

        {
            let clusters = all_possible_clusters(&board, 3);
            assert_clusters_eq(&clusters, &[0x07, 0x0B, 0x0D, 0x0E]);
        }
    }

    #[test]
    fn test_all_possible_clusters_of_3x3() {
        let board = Board::new(3, 3);

        {
            let clusters = all_possible_clusters(&board, 2);
            assert_clusters_eq(
                &clusters,
                &[
                    0x3, 0x6, 0x9, 0x12, 0x18, 0x24, 0x30, 0x48, 0x90, 0xC0, 0x120, 0x180,
                ],
            );
        }

        {
            let clusters = all_possible_clusters(&board, 3);
            assert_clusters_eq(
                &clusters,
                &[
                    0x7, 0xB, 0x13, 0x16, 0x19, 0x1A, 0x26, 0x32, 0x34, 0x38, 0x49, 0x58, 0x92,
                    0x98, 0xB0, 0xC8, 0xD0, 0x124, 0x130, 0x190, 0x1A0, 0x1C0,
                ],
            );
        }

        {
            let clusters = all_possible_clusters(&board, 4);
            assert_clusters_eq(
                &clusters,
                &[
                    0xF, 0x17, 0x1B, 0x1E, 0x27, 0x33, 0x36, 0x39, 0x3A, 0x3C, 0x4B, 0x59, 0x5A,
                    0x78, 0x93, 0x96, 0x99, 0x9A, 0xB2, 0xB4, 0xB8, 0xC9, 0xD2, 0xD8, 0xF0, 0x126,
                    0x132, 0x134, 0x138, 0x192, 0x198, 0x1A4, 0x1B0, 0x1C8, 0x1D0, 0x1E0,
                ],
            );
        }

        {
            let clusters = all_possible_clusters(&board, 5);
            assert_clusters_eq(
                &clusters,
                &[
                    0x1F, 0x2F, 0x37, 0x3B, 0x3D, 0x3E, 0x4F, 0x5B, 0x5E, 0x79, 0x7A, 0x7C, 0x97,
                    0x9B, 0x9E, 0xB3, 0xB6, 0xB9, 0xBA, 0xBC, 0xCB, 0xD3, 0xD6, 0xD9, 0xDA, 0xF2,
                    0xF4, 0xF8, 0x127, 0x133, 0x136, 0x139, 0x13A, 0x13C, 0x178, 0x193, 0x196,
                    0x199, 0x19A, 0x1A6, 0x1B2, 0x1B4, 0x1B8, 0x1C9, 0x1D2, 0x1D8, 0x1E4, 0x1E8,
                    0x1F0,
                ],
            );
        }

        {
            let clusters = all_possible_clusters(&board, 6);
            assert_clusters_eq(
                &clusters,
                &[
                    0x3F, 0x5F, 0x6F, 0x7B, 0x7D, 0x7E, 0x9F, 0xB7, 0xBB, 0xBD, 0xBE, 0xCF, 0xD7,
                    0xDB, 0xDE, 0xF3, 0xF6, 0xF9, 0xFA, 0xFC, 0x12F, 0x137, 0x13B, 0x13D, 0x13E,
                    0x179, 0x17A, 0x17C, 0x197, 0x19B, 0x19E, 0x1A7, 0x1B3, 0x1B6, 0x1B9, 0x1BA,
                    0x1BC, 0x1CB, 0x1D3, 0x1D6, 0x1D9, 0x1DA, 0x1E6, 0x1E9, 0x1EC, 0x1F2, 0x1F4,
                    0x1F8,
                ],
            );
        }

        {
            let clusters = all_possible_clusters(&board, 7);
            assert_clusters_eq(
                &clusters,
                &[
                    0x7F, 0xBF, 0xDF, 0xEF, 0xF7, 0xFB, 0xFD, 0xFE, 0x13F, 0x16F, 0x17B, 0x17D,
                    0x17E, 0x19F, 0x1AF, 0x1B7, 0x1BB, 0x1BD, 0x1BE, 0x1CF, 0x1D7, 0x1DB, 0x1DE,
                    0x1E7, 0x1EB, 0x1ED, 0x1EE, 0x1F3, 0x1F6, 0x1F9, 0x1FA, 0x1FC,
                ],
            );
        }
    }
}
