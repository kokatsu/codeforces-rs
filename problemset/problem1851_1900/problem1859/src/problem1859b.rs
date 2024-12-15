use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();

        let (total, first, second): (u64, u64, u64) =
            (0..n).fold((0, u64::MAX, u64::MAX), |(total, first, second), _| {
                let _m: usize = read();
                let mut a: Vec<u64> = read_vec();

                a.sort();

                (total + a[1], first.min(a[0]), second.min(a[1]))
            });

        let res: u64 = total + first - second;

        writeln!(out, "{}", res).unwrap();
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
