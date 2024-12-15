use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x_min, x_max): (i64, i64) = (0..4).fold((i64::MAX, i64::MIN), |(x_min, x_max), _| {
            let input: Vec<i64> = read_vec();
            let (x, _y): (i64, i64) = (input[0], input[1]);
            (x_min.min(x), x_max.max(x))
        });

        let res: i64 = (x_max - x_min).pow(2);

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
