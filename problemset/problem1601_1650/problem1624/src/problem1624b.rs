use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let a: i64 = input[0];
        let b: i64 = input[1];
        let c: i64 = input[2];

        let res: &str = if (b * 2 - c) % a == 0 && a <= b * 2 - c {
            "YES"
        } else if (b * 2 - a) % c == 0 && c <= b * 2 - a {
            "YES"
        } else if (c - a) % 2 == 0 && ((a + c) / 2) % b == 0 && b <= (a + c) / 2 {
            "YES"
        } else {
            "NO"
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
