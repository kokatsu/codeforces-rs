use std::io::{stdout, Write, BufWriter};

fn main() {
    let _n: usize = read();
    let mut v: Vec<i64> = read_vec();

    v.insert(0, 0);
    let cum_v: Vec<i64> =
        v
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect();

    let mut u: Vec<i64> = v.to_vec();
    u.sort();
    let cum_u: Vec<i64> =
        u
        .iter()
        .scan(0, |cumsum, x| {
            *cumsum += x;
            Some(*cumsum)
        })
        .collect();

    let mut out = BufWriter::new(stdout().lock());

    let m: usize = read();
    for _ in 0..m {
        let input: Vec<usize> = read_vec();
        let t: usize = input[0];
        let l: usize = input[1];
        let r: usize = input[2];

        let res: i64 =
            if t == 1 {
                cum_v[r] - cum_v[l-1]
            }
            else {
                cum_u[r] - cum_u[l-1]
            };

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