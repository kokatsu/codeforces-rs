use std::io::{stdout, BufWriter, Write};

fn is_power_of_two(x: u64) -> bool {
    (x & (x - 1)) == 0
}

fn solve(x: u64, y: u64) -> i64 {
    let d: u64 = y / x;
    if y % x == 0 && is_power_of_two(d) {
        let z: i64 = d.trailing_zeros() as i64;
        return (z + 2) / 3;
    }

    -1
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (u64, u64) = {
            let mut input: Vec<u64> = read_vec();
            input.sort();
            (input[0], input[1])
        };

        let res: i64 = solve(a, b);

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
