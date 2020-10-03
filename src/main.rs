use gerrymandering::{all_possible_clusters, Board};

use colored::Colorize;

pub enum PrintClustersMode {
    Pretty,
    Id,
    None,
}

fn main() {
    print_grid(&random_grid(12, 6, 6));

    // let size = 5_i64;
    // for n in 0..=size.pow(2) {
    //     go(n, size, size, PrintClustersMode::None);
    // }
}

pub fn random_grid(w: usize, h: usize, n: usize) -> Vec<Vec<usize>> {
    use std::collections::HashSet;

    assert_eq!((w * h) % n, 0);
    let cluster_area = w * h / n;

    let mut res = vec![vec![0; w]; h];

    for c in 1..=n {
        let mut boundary = HashSet::new();

        for y in 0..h {
            for x in 0..w {
                if res[y][x] == 0 {
                    boundary.insert((x, y));
                    break;
                }
            }
            if !boundary.is_empty() {
                break;
            }
        }

        let mut area = 0;
        while area < cluster_area {
            let (x, y) = match boundary.iter().next() {
                None => return random_grid(w, h, n),
                Some((x, y)) => (*x, *y),
            };

            if y > 0 && res[y - 1][x] == 0 {
                boundary.insert((x, y - 1));
            }
            if y + 1 < h && res[y + 1][x] == 0 {
                boundary.insert((x, y + 1));
            }
            if x > 0 && res[y][x - 1] == 0 {
                boundary.insert((x - 1, y));
            }
            if x + 1 < w && res[y][x + 1] == 0 {
                boundary.insert((x + 1, y));
            }

            boundary.remove(&(x, y));

            res[y][x] = c;
            area += 1;
        }
    }

    res
}

pub fn print_grid(grid: &[Vec<usize>]) {
    for l in grid {
        for color in l {
            let color = match color {
                1 => " ".on_red(),
                2 => " ".on_green(),
                3 => " ".on_yellow(),
                4 => " ".on_blue(),
                5 => " ".on_magenta(),
                6 => " ".on_cyan(),
                _ => panic!(),
            };
            print!("{}", color);
        }
        println!();
    }
}

pub fn go(n: i64, width: i64, height: i64, mode: PrintClustersMode) {
    let obstacles = Board::new(width, height);
    let mut clusters = all_possible_clusters(&obstacles, n);
    clusters.sort_by_key(Board::board_id);

    println!(
        "*** found {} clusters of size {} in {}x{} matrix ***",
        clusters.len(),
        n,
        width,
        height
    );

    match mode {
        PrintClustersMode::None => {}
        PrintClustersMode::Pretty => {
            for cluster in clusters {
                for y in 0..cluster.height() {
                    let line: String = (0..cluster.width())
                        .map(|x| if cluster.get(x, y) { 'X' } else { '.' })
                        .collect();

                    println!("{}", line);
                }

                println!();
            }
            println!();
        }
        PrintClustersMode::Id => {
            for cluster in clusters {
                println!("cluster id={id} idhex=0x{id:X}", id = cluster.board_id());
            }
        }
    }
}
