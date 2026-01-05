use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (_n, _q): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let mut a: Vec<u32> = read_vec();
        let x: Vec<u32> = read_vec();

        let mut u: Vec<u32> = Vec::new();
        let mut s: HashSet<u32> = HashSet::new();
        for &v in x.iter() {
            if !s.contains(&v) {
                s.insert(v);
                u.push(v);
            }
        }

        for &i in u.iter() {
            if !s.contains(&i) {
                continue;
            }

            let d: u32 = 2u32.pow(i);
            for v in a.iter_mut() {
                if *v % d == 0 {
                    *v += d / 2;
                }
            }
        }

        let res: String = a
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");

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
