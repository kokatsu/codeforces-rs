use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();

    let mut a: Vec<u32> = vec![0; n];
    for x in a.iter_mut() {
        let (_l, r): (u32, u32) = {
            let input = read_vec::<u32>();
            (input[0], input[1])
        };
        *x = r;
    }

    let k: u32 = read();

    let res: usize = a.iter().filter(|&&x| x >= k).count();

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
