use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let letters: Vec<char> = "abcdefgh".chars().collect();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let a: Vec<char> = read_string().chars().collect();
        let n: u32 = a[1].to_digit(10).unwrap();

        let res: String = letters.iter().fold(String::new(), |s, &l| {
            if l == a[0] {
                let t: String = (1..=8).fold(String::new(), |t, i| {
                    if i == n {
                        t
                    } else {
                        t + " " + &l.to_string() + &i.to_string()
                    }
                });
                s + " " + t.to_string().trim()
            } else {
                s + " " + &l.to_string() + &a[1].to_string()
            }
        });

        writeln!(out, "{}", res.trim()).unwrap();
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
