use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();
    for _ in 0..t {
        let (n, m): (i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1])
        };

        let u: i32 = n.min(m);
        let v: i32 = n.max(m);

        let res: i32 =
            if u == 1 && v >= 3 {
                -1
            }
            else {
                v * 2 - (u + v) % 2 - 2
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