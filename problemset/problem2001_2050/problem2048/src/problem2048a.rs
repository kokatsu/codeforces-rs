use std::collections::{HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

fn solve() -> String {
    let x: u32 = read();

    let mut set: HashSet<u32> = HashSet::new();
    let mut deq: VecDeque<u32> = VecDeque::new();
    deq.push_back(x);
    while let Some(f) = deq.pop_front() {
        if f % 33 == 0 {
            return "YES".to_string();
        }

        if f < 33 {
            continue;
        }

        let mut g: u32 = f;
        let mut l: u32 = 0;
        let mut b: u32 = 1;
        while g > 0 {
            let m: u32 = g % 100;
            if m == 33 {
                let r: u32 = (g / 100) * b + l;
                if !set.contains(&r) {
                    deq.push_back(r);
                    set.insert(r);
                }
            }
            l += (g % 10) * b;
            b *= 10;
            g /= 10;
        }
    }

    "NO".to_string()
}

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let res: String = solve();

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
