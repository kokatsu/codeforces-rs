use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (p1, p2, p3): (i8, i8, i8) = {
            let input: Vec<i8> = read_vec();
            (input[0], input[1], input[2])
        };

        let res: i8 = match ((p1 % 2 + p2 % 2 + p3 % 2) % 2 == 0, p1 + p2 >= p3) {
            (true, true) => (p1 + p2 + p3) / 2,
            (true, false) => p1 + p2,
            _ => -1,
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