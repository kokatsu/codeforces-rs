use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, x): (usize, u64) = {
            let input: Vec<u64> = read_vec();
            (input[0] as usize, input[1])
        };

        let a: Vec<u64> = read_vec();

        let (mut ok, mut ng): (u64, u64) = (0, 10_u64.pow(12));
        while ng - ok > 1 {
            let mid: u64 = (ok + ng) / 2;
            let num: u64 = a
                .iter()
                .map(|u| if u < &mid { mid - u } else { 0 })
                .sum();
            if num <= x {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }

        let res: u64 = ok;

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