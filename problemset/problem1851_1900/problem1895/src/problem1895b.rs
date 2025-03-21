use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<u32> = read_vec();

        a.sort();

        let length: u32 = a[n - 1] + a[n * 2 - 1] - a[0] - a[n];
        let points = (0..n).fold(String::new(), |points: String, i| {
            points + "\n" + &format!("{} {}", a[i], a[n + i])
        });

        let res: String = format!("{}{}", length, points);

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
