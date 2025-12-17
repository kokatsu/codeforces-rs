use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m): (u64, u64) = {
            let input = read_vec::<u64>();
            (input[0], input[1])
        };

        let k: u64 = n / m;
        let (d, r): (u64, u64) = (k / 10, k % 10);

        let res: u64 = (0..10)
            .map(|i| (i * m) % 10 * (d + if i <= r { 1 } else { 0 }))
            .sum();

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
