use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let n: usize = read();
    let a: Vec<u16> = read_vec();

    let (t, steps): (u16, String) = (0..n - 1).fold((0, "".to_string()), |(t, steps), i| {
        if a[i] >= a[i + 1] {
            (t + 1, format!("{} {}", steps, a[i]))
        } else {
            (t, steps)
        }
    });

    let res: String = format!(
        "{}\n{}",
        t + 1,
        (steps + " " + &a[n - 1].to_string()).trim_start()
    );

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
