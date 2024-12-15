use std::io::{stdout, BufWriter, Write};

const L: i64 = 10;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: i64 = read();

        let mut l: i64 = L;
        let mut res: i64 = 0;
        while n >= l {
            res += L - 1;
            l *= L;
        }

        let mut ord: i64 = 1;
        while ord * L + 1 < l {
            ord *= L;
            ord += 1;
        }

        let mut num: i64 = ord;
        while num <= n {
            res += 1;
            num += ord;
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
