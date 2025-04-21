use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: u16 = read();

    let (mut a, mut b): (u16, u16) = (0, 1);
    let mut fib: HashSet<usize> = HashSet::new();
    while b <= n {
        fib.insert(b as usize);
        let c = a + b;
        a = b;
        b = c;
    }

    let res: String = (1..=n)
        .map(|i| {
            if fib.contains(&(i as usize)) {
                'O'
            } else {
                'o'
            }
        })
        .collect::<String>();

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
