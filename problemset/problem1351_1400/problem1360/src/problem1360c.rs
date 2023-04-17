use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: i64 = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<i64> = read_vec();

        a.sort();

        let mut even: i64 = 0;
        let mut odd: i64 = 0;
        let mut diff: i64 = 0;
        for i in 0..n {
            if a[i] % 2 == 0 {
                even += 1;
            }
            else {
                odd += 1;
            }

            if i > 0 && a[i] - a[i-1] == 1 {
                diff += 1;
            }
        }

        let res: &str = match (even%2, odd%2, diff>=1) {
            (0, 0, _) => "Yes",
            (1, 1, true) => "Yes",
            (1, 1, false) => "No",
            _ => "No",
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