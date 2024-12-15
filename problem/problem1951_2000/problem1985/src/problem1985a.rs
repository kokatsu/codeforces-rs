use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (a, b): (&str, &str) = {
            let input: Vec<String> = read_vec();
            (&input[0].clone(), &input[1].clone())
        };

        let u: Vec<char> = a.to_string().chars().collect();
        let v: Vec<char> = b.to_string().chars().collect();

        let x: String = u.clone()[1..3].iter().collect();
        let y: String = v.clone()[1..3].iter().collect();

        let res: String = format!("{}{} {}{}", v[0], x, u[0], y);

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
