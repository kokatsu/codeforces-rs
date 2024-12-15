use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let v: Vec<u8> = s.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut l: usize = 0;
        let mut r: usize = v.len() - 1;
        let mut now: u8 = ('a' as u8) + (r as u8);

        let mut res: &str = "YES";
        while l <= r {
            if v[l] == now {
                l += 1;
                now -= 1;
            } else if v[r] == now {
                r -= 1;
                now -= 1;
            } else {
                res = "NO";
                break;
            }
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
