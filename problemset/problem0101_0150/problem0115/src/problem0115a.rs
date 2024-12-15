use std::io::{stdout, BufWriter, Write};

fn dfs(x: usize, y: &Vec<i64>, z: &mut Vec<i64>) {
    let pos: i64 = y[x];
    if pos == -1 {
        z[x] = 1;
    } else if z[x] == !0 {
        dfs((pos - 1) as usize, y, z);
        z[x] = z[(pos - 1) as usize] + 1;
    }
}

fn main() {
    let n: usize = read();

    let mut p: Vec<i64> = vec![0; n];
    for i in 0..n {
        p[i] = read();
    }

    let mut c: Vec<i64> = vec![!0; n];
    for i in 0..n {
        dfs(i, &p, &mut c);
    }

    let res: i64 = *c.iter().max().unwrap();

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
