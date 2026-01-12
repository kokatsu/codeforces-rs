use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (h1, m1): (i16, i16) = {
        let input: Vec<i16> = read_string()
            .split(':')
            .map(|v| v.parse().unwrap())
            .collect();
        (input[0], input[1])
    };

    let (h2, m2): (i16, i16) = {
        let input: Vec<i16> = read_string()
            .split(':')
            .map(|v| v.parse().unwrap())
            .collect();
        (input[0], input[1])
    };

    let d: i16 = (60 * (h2 - h1) + (m2 - m1)) / 2;

    let (h3, m3): (i16, i16) = (h1 + (m1 + d) / 60, (m1 + d) % 60);

    let res: String = format!("{:02}:{:02}", h3, m3);

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
