use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let b: Vec<u64> = read_vec();

        let (a, cnt): (String, u64) = (1..n).fold((b[0].to_string(), 1), |(a, cnt), i| {
            if b[i] < b[i - 1] {
                (a + " 1 " + &b[i].to_string(), cnt + 2)
            } else {
                (a + " " + &b[i].to_string(), cnt + 1)
            }
        });

        writeln!(out, "{}\n{}", cnt, a).unwrap();
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
