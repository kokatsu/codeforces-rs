use std::io::{stdout, BufWriter, Write};

fn main() {
    let input: Vec<i64> = read_vec();
    let n: i64 = input[0];
    let m: i64 = input[1];

    let l: i64 = n.max(m);

    let mut res: i64 = 0;
    for a in 0..=l {
        for b in 0..=l {
            let x: i64 = a * a + b;
            let y: i64 = a + b * b;

            if x == n && y == m {
                res += 1;
            }

            if x > n || y > m {
                break;
            }
        }
    }

    let mut out = BufWriter::new(stdout().lock());
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
