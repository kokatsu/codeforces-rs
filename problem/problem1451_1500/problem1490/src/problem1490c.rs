use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

const N: i64 = 10000i64;

fn main() {
    let t: usize = read();

    let mut set: HashSet<i64> = HashSet::new();
    for i in 1..=N {
        set.insert(i * i * i);
    }

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let x: i64 = read();

        let mut is_ok: bool = false;
        for i in 1..=N {
            let a: i64 = i * i * i;

            if a >= x {
                break;
            }

            if set.contains(&(x - a)) {
                is_ok = true;
                break;
            }
        }

        let res: &str = if is_ok { "YES" } else { "NO" };

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
