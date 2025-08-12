use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let words: Vec<String> = read_vec();

        let res: String = words[1..words.len()]
            .iter()
            .fold(words[0].clone(), |res, w| {
                let c: Vec<char> = w.chars().collect();
                if res.chars().last() == c.first().copied() {
                    res + &c[1..].iter().collect::<String>()
                } else {
                    res + w
                }
            });

        let res: String = if res.len() == n { res } else { res + "a" };

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
