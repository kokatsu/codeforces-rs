use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let t: usize = read();

    for _ in 0..t {
        let s: String = read_string();

        let t: Vec<&str> = s.split(":").collect();

        let h: i64 = t[0].parse::<i64>().ok().unwrap();
        let m: &str = t[1];

        let res: String = match h {
            0 => format!("12:{} AM", m),
            12 => format! {"12:{} PM", m},
            h if h < 12 => format!("{:02}:{} AM", h, m),
            _ => format!("{:02}:{} PM", h - 12, m),
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
