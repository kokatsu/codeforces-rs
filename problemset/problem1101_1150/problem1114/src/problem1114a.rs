use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (x, y, z): (u32, u32, u32) = {
        let input = read_vec::<u32>();
        (input[0], input[1], input[2])
    };

    let (a, b, c): (u32, u32, u32) = {
        let input = read_vec::<u32>();
        (input[0], input[1], input[2])
    };

    let res: &str = if x <= a && (x + y) <= (a + b) && (x + y + z) <= (a + b + c) {
        "YES"
    } else {
        "NO"
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
