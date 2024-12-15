use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<String> = (0..n).map(|_| read_string()).collect::<Vec<String>>();

        let set: HashSet<String> = s.iter().cloned().collect();

        let mut res: String = String::new();
        for x in s.iter() {
            let mut is_ok: bool = false;

            let l: usize = x.len();
            for i in 0..l {
                if set.contains(&x[0..i]) && set.contains(&x[i..l]) {
                    is_ok = true;
                    break;
                }
            }

            res += if is_ok { "1" } else { "0" };
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
