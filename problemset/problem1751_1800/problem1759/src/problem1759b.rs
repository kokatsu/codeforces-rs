use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_m, s): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let b: Vec<usize> = read_vec();

        let sum: usize = b.iter().sum::<usize>() + s;
        let max: usize = *b.iter().max().unwrap();

        let res: &str = if sum < max {
            "NO"
        } else {
            let mut ok: usize = 0;
            let mut ng: usize = sum;
            while ng - ok > 1 {
                let mid: usize = (ok + ng) / 2;
                if mid * (mid + 1) / 2 <= sum {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            if ok >= max && ok * (ok + 1) / 2 == sum {
                "YES"
            } else {
                "NO"
            }
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
