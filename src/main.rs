use gerrymandering::{all_possible_clusters, Board};

fn main() {
    let size = 3_i64;

    for n in 0..=size.pow(2) {
        go(n, size);
    }
}

pub fn go(n: i64, size: i64) {
    let obstacles = Board::new(size, size);
    let clusters = all_possible_clusters(&obstacles, n);

    println!(
        "*** found {} clusters of size {} in {}x{} matrix ***",
        clusters.len(),
        n,
        size,
        size
    );

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
