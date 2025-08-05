use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let _n: usize = read();
        let a: Vec<i64> = read_vec();

        let (min, max) = a.iter().fold((i64::MAX, i64::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

        let mut set: HashSet<i64> = HashSet::new();
        for &x in &a {
            set.insert(x);
        }

        let m: i64 = min * max;
        let mut d: i64 = 2;
        let mut ok: bool = true;
        while d * d <= m {
            if m % d == 0 {
                if !set.contains(&d) || !set.contains(&(m / d)) {
                    ok = false;
                    break;
                }

                set.remove(&d);
                if d * d != m {
                    set.remove(&(m / d));
                }
            }
            d += 1;
        }

        let res: i64 = if ok && set.is_empty() { m } else { -1 };

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
