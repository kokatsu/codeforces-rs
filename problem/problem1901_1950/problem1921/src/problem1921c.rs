use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let input: Vec<i64> = read_vec();
        let (_n, f, a, b): (usize, i64, i64, i64) =
            (input[0] as usize, input[1], input[2], input[3]);
        let m: Vec<i64> = read_vec();

        let res: &str = m
            .iter()
            .fold(("YES", f, 0), |(res, rem, pre), &x| {
                let min = b.min((x - pre) * a);
                if rem > min {
                    (res, rem - min, x)
                } else {
                    ("NO", rem - min, x)
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
