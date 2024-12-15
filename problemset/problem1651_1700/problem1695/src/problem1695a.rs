use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let (n, m): (usize, usize) = {
            let input: Vec<usize> = read_vec();
            (input[0], input[1])
        };

        let mut max: i32 = i32::MIN;
        let (mut u, mut v): (usize, usize) = (0, 0);
        for i in 0..n {
            let a: Vec<i32> = read_vec();
            for (j, &x) in a.iter().enumerate() {
                if x > max {
                    max = x;
                    (u, v) = (i, j);
                }
            }
        }

        let res: usize = (u + 1).max(n - u) * (v + 1).max(m - v);

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
