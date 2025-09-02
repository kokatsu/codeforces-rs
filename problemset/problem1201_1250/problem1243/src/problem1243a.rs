use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let k: usize = read();

    for _ in 0..k {
        let n: usize = read();
        let mut a: Vec<usize> = read_vec();

        a.sort_by(|x, y| y.cmp(x));

        let mut i: usize = 0;
        let mut res: usize = 0;
        while i < n {
            if res < a[i] {
                res += 1;
                i += 1;
            } else {
                break;
            }
        }

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
