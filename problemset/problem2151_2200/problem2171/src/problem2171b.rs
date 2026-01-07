use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<i32> = read_vec();

        if a[0] == -1 || a[n - 1] == -1 {
            let m: i32 = a[0].max(a[n - 1]);
            a[0] = m;
            a[n - 1] = m;
        }

        a.iter_mut().for_each(|x| {
            if *x == -1 {
                *x = 0;
            }
        });

        let v: u32 = a[0].abs_diff(a[n - 1]);
        let b: String = a
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let res: String = format!("{}\n{}", v, b);

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
