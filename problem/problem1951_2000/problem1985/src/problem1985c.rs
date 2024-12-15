use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<u64> = read_vec();

        let res: u64 = a
            .into_iter()
            .fold((0, 0, 0), |(res, max, sum), x| {
                if x >= max {
                    let num: u64 = sum + max;
                    if x == num {
                        (res + 1, x, num)
                    }
                    else {
                        (res, x, num)
                    }
                }
                else {
                    let num: u64 = sum + x;
                    if max == num {
                        (res + 1, max, num)
                    }
                    else {
                        (res, max, num)
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