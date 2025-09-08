use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let h: usize = n.div_ceil(2);

    let res: String = (1..=n)
        .map(|i| {
            (1..=n)
                .map(|j| {
                    let x = i.min(n - i + 1);
                    let y = j.min(n - j + 1);
                    if x + y > h {
                        'D'
                    } else {
                        '*'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

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
