use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let (total, one, two): (i64, i64, i64) =
            a
            .iter()
            .fold((0, 0, 0), |(total, one, two), x| {
                if *x == 1 {
                    (total+x, one+1, two)
                }
                else {
                    (total+x, one, two+1)
                }
            });

        let d: i64 = total / 2;
        let r: i64 = total % 2;

        let mut is_ok: bool = false;
        for i in 0..=two {
            let rem: i64 = d - i * 2;
            if rem < 0 {
                break;
            }

            is_ok |= rem <= one;
        }
        is_ok &= r == 0;

        let res: &str =
            if is_ok {
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