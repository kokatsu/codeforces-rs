use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<i64> = read_vec();
    let x1: i64 = input[0];
    let y1: i64 = input[1];
    let x2: i64 = input[2];
    let y2: i64 = input[3];

    let dx: i64 = (x1 - x2).abs();
    let dy: i64 = (y1 - y2).abs();

    let mut out = BufWriter::new(stdout().lock());

    if dx != 0 && dy != 0 && dx != dy {
        writeln!(out, "-1").unwrap();
        return;
    }

    let (x3, y3, x4, y4): (i64, i64, i64, i64) = match (dx == 0, dy == 0) {
        (true, false) => (x1 + dy, y1, x2 + dy, y2),
        (false, true) => (x1, y1 + dx, x2, y2 + dx),
        _ => (x1, y2, x2, y1),
    };

    writeln!(out, "{} {} {} {}", x3, y3, x4, y4).unwrap();
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
