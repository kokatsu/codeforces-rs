use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<u64> = read_vec();
        let mut a: u64 = input[0];
        let k: u64 = input[1];

        for _ in 1..k {
            let mut minmax: (u64, u64) = (9, 0);
            let mut r: u64 = a;
            while r > 0 {
                let d: u64 = r % 10;
                minmax = (minmax.0.min(d), minmax.1.max(d));
                r /= 10;
            }

            if minmax.0 == 0 {
                break;
            }

            a += minmax.0 * minmax.1;
        }

        writeln!(out, "{}", a).unwrap();
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
