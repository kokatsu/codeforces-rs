use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let input: Vec<usize> = read_vec();
    let (_n, _m): (usize, usize) = (input[0], input[1]);

    let a: Vec<u64> = read_vec();

    let cumsum: Vec<u64> = a
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect();

    let b: Vec<u64> = read_vec();

    for y in b.into_iter() {
        let l: usize = cumsum.partition_point(|&x| x <= y);

        let (dormitory, room): (usize, u64) = if l == 0 {
            (l + 1, y)
        } else if y == cumsum[l - 1] {
            (l, a[l - 1])
        } else {
            (l + 1, a[l] + y - cumsum[l])
        };

        writeln!(out, "{} {}", dormitory, room).unwrap();
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
