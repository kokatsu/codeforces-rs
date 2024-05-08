use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let q: usize = read();

    for _ in 0..q {
        let (_n, t): (i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0], input[1])
        };

        let a: Vec<i64> = read_vec();
        let b: Vec<i64> = read_vec();

        let res: i64 = a
            .into_iter()
            .zip(b.into_iter())
            .enumerate()
            .fold((-1, -1), |(res, num), (i, (x, y))| {
                let j: i64 = i as i64;
                if j + x <= t && y > num {
                    (j+1, y)
                }
                else {
                    (res, num)
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