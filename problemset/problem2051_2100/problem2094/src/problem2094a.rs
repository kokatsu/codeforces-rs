use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (s1, s2, s3): (String, String, String) = {
            let input: Vec<String> = read_vec();
            (input[0].clone(), input[1].clone(), input[2].clone())
        };

        let c1: char = s1.chars().next().unwrap();
        let c2: char = s2.chars().next().unwrap();
        let c3: char = s3.chars().next().unwrap();

        let res: String = format!("{}{}{}", c1, c2, c3);

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
