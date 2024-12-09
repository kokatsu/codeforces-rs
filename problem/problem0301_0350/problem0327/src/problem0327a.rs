use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let a: Vec<i64> = read_vec();

    let mut res: i64 = 0;
    for i in 0..n {
        for j in i..n {
            let num = (0..n).fold(0, |num, k| {
                                if i <= k && k <= j {
                                    num + 1 - a[k]
                                }
                                else {
                                    num + a[k]
                                }
                            });

            res = res.max(num);
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