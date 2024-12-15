use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let n: i64 = input[0];
        let k: i64 = input[1];

        if n - k + 1 > 0 && (n - k + 1) % 2 == 1 {
            writeln!(out, "YES").unwrap();
            writeln!(
                out,
                "{} {}",
                n - k + 1,
                (0..k - 1).fold(String::new(), |x, _| x + &1.to_string() + " ")
            )
            .unwrap();
        } else if n - (k - 1) * 2 > 0 && (n - (k - 1) * 2) % 2 == 0 {
            writeln!(out, "YES").unwrap();
            writeln!(
                out,
                "{} {}",
                n - (k - 1) * 2,
                (0..k - 1).fold(String::new(), |x, _| x + &2.to_string() + " ")
            )
            .unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
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
