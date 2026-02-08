use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (n, m): (usize, usize) = {
        let input: Vec<usize> = read_vec();
        (input[0], input[1])
    };

    let mut s: Vec<String> = Vec::with_capacity(n);
    let (mut a, mut b, mut c, mut d): (usize, usize, usize, usize) = (n, 0, m, 0);
    for i in 0..n {
        let line: String = read_string();
        for (j, x) in line.chars().enumerate() {
            if x == '*' {
                a = a.min(i);
                b = b.max(i);
                c = c.min(j);
                d = d.max(j);
            }
        }
        s.push(line);
    }

    let mut t: Vec<String> = Vec::with_capacity(b - a + 1);
    for x in s.iter().take(b + 1).skip(a) {
        t.push(x[c..=d].to_string());
    }

    let res: String = t.join("\n");

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
