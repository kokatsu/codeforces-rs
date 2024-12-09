use std::io::{stdout, BufWriter, Write};

fn f(x: i64, y: i64, z: i64, res: &mut i64) {
    if y == z {
        if *res == -1 || *res < x {
            *res = x;
        }
    }

    if y * 2 <= z {
        f(x + 1, y * 2, z, res);
    }
    if y * 3 <= z {
        f(x + 1, y * 3, z, res);
    }
}

fn main() {
    let input: Vec<i64> = read_vec();
    let n: i64 = input[0];
    let m: i64 = input[1];

    let mut res: i64 = -1;

    f(0, n, m, &mut res);

    let mut out = BufWriter::new(stdout().lock());
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
