use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();
        let a: Vec<i64> = read_vec();

        let h: i64 = n / 2;
        let m: i64 = n - h;

        let mut odd_count: i64 = 0;
        let mut even_count: i64 = 0;
        for (i, x) in a.iter().enumerate() {
            if i % 2 == 0 {
                if x % 2 == 0 {
                    odd_count += 1;
                }
            } else {
                if x % 2 == 0 {
                    even_count += 1;
                }
            }
        }

        let res: &str =
            if (odd_count == 0 || odd_count == m) && (even_count == 0 || even_count == h) {
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
