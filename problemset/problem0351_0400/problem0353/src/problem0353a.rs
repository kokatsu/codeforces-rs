use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let (mut u, mut l): (i8, i8) = (0, 0);
    let mut odd: i8 = 0;

    for _ in 0..n {
        let (x, y): (i8, i8) = {
            let input: Vec<i8> = read_vec();
            (input[0], input[1])
        };

        let x: i8 = x % 2;
        let y: i8 = y % 2;

        u += x;
        l += y;

        if x != y {
            odd += 1;
        }
    }

    let res: i8 = if (u % 2 == 0) && (l % 2 == 0) {
        0
    } else if (u % 2 == 1) && (l % 2 == 1) && (odd > 0) {
        1
    } else {
        -1
    };

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
