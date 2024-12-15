use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let a: Vec<u64> =
        (1..=5000)
        .step_by(2)
        .scan(0, |cum, i| {
            *cum += i;
            Some(*cum)
        })
        .collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: u64 = read();

        let res: usize = a.partition_point(|&x| x < s) + 1;

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