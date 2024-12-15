use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<usize> = read_vec();
        let (n, q): (usize, usize) = (input[0], input[1]);

        let mut a: Vec<u64> = read_vec();

        a.sort_by(|x, y| x.cmp(y).reverse());

        let cumsum: Vec<u64> = a
            .iter()
            .scan(0, |cumsum, x| {
                *cumsum += x;
                Some(*cumsum)
            })
            .collect();

        for _ in 0..q {
            let x: u64 = read();

            if x > cumsum[n - 1] {
                writeln!(out, "-1").unwrap();
                continue;
            }

            let res: usize = cumsum.partition_point(|&c| c < x) + 1;

            writeln!(out, "{}", res).unwrap();
        }
    }
}

#[allow(dead_code)]
fn read_string() -> String {
    let mut s: String = String::new();
    std::io::stdin().read_line(&mut s).ok();

    s.trim().to_string()
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    read_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read_string()
        .split_whitespace()
        .map(|v| v.parse().ok().unwrap())
        .collect()
}
