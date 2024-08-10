use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, s, m): (u32, u32, u32) = {
            let input: Vec<u32> = read_vec();
            (input[0], input[1], input[2])
        };

        let res: &str = (0..n)
            .fold(("NO", 0), |(res, pre), i| {
                let (l, r): (u32, u32) = {
                    let input: Vec<u32> = read_vec();
                    (input[0], input[1])
                };
                if i == n - 1 {
                    if s <= l - pre || s <= m - r {
                        ("YES", r)
                    }
                    else {
                        (res, r)
                    }
                }
                else {
                    if s <= l - pre {
                        ("YES", r)
                    }
                    else {
                        (res, r)
                    }
                }
            })
            .0;

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