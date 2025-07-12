use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b, c, d): (u64, u64, u64, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0], input[1], input[2], input[3])
        };

        let x: u64 = a * d;
        let y: u64 = b * c;

        let res: u8 = if (a == 0 && c == 0) || x == y {
            0
        } else if a == 0 || c == 0 || x % y == 0 || y % x == 0 {
            1
        } else {
            2
        };

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
