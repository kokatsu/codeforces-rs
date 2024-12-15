use std::io::{stdout, BufWriter, Write};

fn solve() -> i64 {
    let n: usize = read();
    let (ps, pe): (i64, i64) = {
        let input: Vec<i64> = read_vec();
        (input[0], input[1])
    };

    let mut ret = ps;
    for _ in 1..n {
        let (s, e): (i64, i64) = {
            let input: Vec<i64> = read_vec();
            (input[0], input[1])
        };

        if s >= ps && e >= pe {
            ret = -1;
        }
    }

    ret
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let res: i64 = solve();

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
