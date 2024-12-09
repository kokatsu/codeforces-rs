use std::io::{stdout, Write, BufWriter};

fn main() {
    let n: usize = read();
    let a: Vec<i64> = read_vec();

    let cumsum: Vec<i64> =
        a
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect();

    let res: i64 =
        if cumsum[n-1] % 3 == 0 && n >= 3 {
            let counts: Vec<i64> =
                cumsum
                .iter()
                .map(|&x| x == cumsum[n-1] / 3)
                .collect::<Vec<bool>>()
                .iter()
                .scan(0, |cum, x| {
                    if *x {
                        *cum += 1;
                    }
                    Some(*cum)
                })
                .collect();
            (2..n)
            .rev()
            .fold((0, 0), |(res, cum), i| {
                if cum + a[i] == cumsum[n-1] / 3 {
                    (res+counts[i-2], cum+a[i])
                }
                else {
                    (res, cum+a[i])
                }
            })
            .0
        }
        else {
            0
        };

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