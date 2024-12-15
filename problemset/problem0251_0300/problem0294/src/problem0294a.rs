use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let mut a: Vec<i32> = read_vec();

    let m: usize = read();
    for _ in 0..m {
        let (x, y): (usize, i32) = {
            let input: Vec<usize> = read_vec();
            (input[0] - 1, input[1] as i32)
        };

        if x > 0 {
            a[x - 1] += y - 1;
        }

        if x < n - 1 {
            a[x + 1] += a[x] - y;
        }

        a[x] = 0;
    }

    let res: String = a
        .iter()
        .map(|x| x.to_string())
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
