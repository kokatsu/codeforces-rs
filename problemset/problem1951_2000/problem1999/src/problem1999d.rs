use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let mut s: Vec<char> = read_string().chars().collect();
        let t: Vec<char> = read_string().chars().collect();

        let l: usize = t.len();

        let mut is_ok: bool = false;
        let mut index: usize = 0;
        for x in s.iter_mut() {
            if *x == '?' {
                *x = t[index];
                index += 1;
            }
            else if *x == t[index] {
                index += 1;
            }

            if index >= l {
                is_ok = true;
                index = 0;
            }
        }

        let res: String =
            if is_ok {
                "YES\n".to_string() + &s.iter().collect::<String>()
            }
            else {
                "NO".to_string()
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