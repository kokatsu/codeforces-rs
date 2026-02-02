use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (mut a, mut b): (u64, u64) = {
        let input: Vec<u64> = read_vec();
        (input[0], input[1])
    };

    let (x, y, z): (u64, u64, u64) = {
        let input: Vec<u64> = read_vec();
        (input[0], input[1], input[2])
    };

    let mut res: u64 = 0;

    if a >= x * 2 {
        a -= x * 2;
    } else {
        res += (x * 2) - a;
        a = 0;
    }

    if b >= z * 3 {
        b -= z * 3;
    } else {
        res += (z * 3) - b;
        b = 0;
    }

    if a < y {
        res += y - a;
    }

    if b < y {
        res += y - b;
    }

    writeln!(out, "{}", res).unwrap();
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
