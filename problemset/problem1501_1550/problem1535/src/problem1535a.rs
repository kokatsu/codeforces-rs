use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let mut a: Vec<i64> = read_vec();

        for i in 0..2 {
            if a[i*2] < a[i*2+1] {
                a.swap(i*2, i*2+1);
            }
        }

        let res: &str =
            if a[0] > a[3] && a[2] > a[1] {
                "YES"
            }
            else {
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