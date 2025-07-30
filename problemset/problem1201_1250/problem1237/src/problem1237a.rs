use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let b: Vec<i32> = (0..n)
        .fold((Vec::new(), 0, 0), |(mut b, p, m), _| {
            let a: i32 = read();
            if a % 2 == 0 {
                b.push(a / 2);
                (b, p, m)
            } else if a > 0 {
                let d: i32 = (a + p) / 2;
                let r: i32 = (a + p) % 2;
                b.push(d);
                (b, r, m)
            } else {
                let d: i32 = (a + m) / 2;
                let r: i32 = (a + m) % 2;
                b.push(d);
                (b, p, r)
            }
        })
        .0;

    let res: String = b
        .iter()
        .map(|v| v.to_string())
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
