use std::cmp::Ordering;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, k): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let s: Vec<char> = read_string().chars().collect();

        let (z, o): (usize, usize) =
            (0..n / 2).fold((0, 0), |(z, o), i| match (s[i], s[n - 1 - i]) {
                ('0', '0') => (z + 1, o),
                ('1', '1') => (z, o + 1),
                _ => (z, o),
            });

        let m: usize = z + o;

        let res: &str = match m.cmp(&k) {
            Ordering::Equal => "YES",
            Ordering::Less => {
                if (k - m) % 2 == 0 {
                    "YES"
                } else {
                    "NO"
                }
            }
            Ordering::Greater => {
                let d: usize = (m - k) / 2;
                let r: usize = (m - k) % 2;
                if r == 0 && z >= d && o >= d {
                    "YES"
                } else {
                    "NO"
                }
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
