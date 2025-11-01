use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let res: &str = (0..n).fold("NO", |acc, _| {
        let (_name, before, after): (String, i16, i16) = {
            let input: Vec<String> = read_vec();
            (
                input[0].clone(),
                input[1].parse().ok().unwrap(),
                input[2].parse().ok().unwrap(),
            )
        };
        if before >= 2400 && after > before {
            "YES"
        } else {
            acc
        }
    });

    writeln!(out, "{}", res).unwrap();
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
