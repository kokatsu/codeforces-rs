use std::io::{stdout, BufWriter, Write};

fn main() {
    let t: usize = read();

    let rgb: Vec<char> = vec!['r', 'g', 'b'];

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let s: String = read_string();

        let res: &str = rgb.iter().fold("YES", |res, x| {
            let l: usize = s.find(*x).unwrap();
            let u: usize = s.find(x.to_ascii_uppercase()).unwrap();
            if l < u {
                res
            } else {
                "NO"
            }
        });

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
