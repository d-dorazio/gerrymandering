use gerrymandering::{all_possible_clusters, Board};

pub enum PrintClustersMode {
    Pretty,
    Id,
    None,
}

fn main() {
    let size = 5_i64;
    for n in 0..=size.pow(2) {
        go(n, size, size, PrintClustersMode::None);
    }
}

fn go(n: i64, width: i64, height: i64, mode: PrintClustersMode) {
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
