use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m, r, c): (u64, u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let (il, iu): (u64, u64) = (r - 1, n - r);
        let (jl, ju): (u64, u64) = (c - 1, m - c);

        let res: u64 = il.max(iu) + jl.max(ju);

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
