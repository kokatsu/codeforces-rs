use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let n: usize = read();
        let mut a: Vec<u32> = read_vec();

        let mut num: u32 = 0;
        for i in 1..n - 1 {
            if a[i] <= a[i - 1] || a[i] <= a[i + 1] {
                continue;
            }

            a[i + 1] = if i < n - 2 { a[i].max(a[i + 2]) } else { a[i] };

            num += 1;
        }

        let res: String = format!(
            "{}\n{}",
            num,
            a.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

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
