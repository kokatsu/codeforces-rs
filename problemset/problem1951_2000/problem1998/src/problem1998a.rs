use std::io::{stdout, Write, BufWriter};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (x, y, k): (i32, i32, i32) = {
            let input: Vec<i32> = read_vec();
            (input[0], input[1], input[2])
        };

        let f: String = (1..=k/2)
            .map(|i| format!("{} {}\n{} {}", x - i, y, x + i, y))
            .collect::<Vec<String>>()
            .join("\n");

        let b: String =
            if k % 2 == 1 {
                let z: &str = if f.is_empty() { "" } else { "\n" };
                format!("{}{} {}", z, x, y)
            }
            else {
                String::new()
            };

        let res: String = f + &b;

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