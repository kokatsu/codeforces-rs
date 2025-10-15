use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut n: usize = read();

    // 0 <= x < 10
    let f = |x: usize| -> String {
        let d: usize = x / 5;
        let m: usize = x % 5;
        let l: String = if d == 0 { "O-" } else { "-O" }.to_string();
        let r: String = (0..5)
            .map(|i| if i == m { '-' } else { 'O' })
            .collect::<String>();
        l + "|" + &r
    };

    let mut v: Vec<String> = Vec::new();
    if n == 0 {
        v.push(f(0));
    }
    while n > 0 {
        v.push(f(n % 10));
        n /= 10;
    }

    let res: String = v.into_iter().collect::<Vec<String>>().join("\n");

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
