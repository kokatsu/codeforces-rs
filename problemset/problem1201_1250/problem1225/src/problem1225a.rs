use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (da, db): (u8, u8) = {
        let input: Vec<u8> = read_vec();
        (input[0], input[1])
    };

    let res: String = if da == db {
        format!("{}0 {}1", da, db)
    } else if da + 1 == db {
        format!("{}9 {}0", da, db)
    } else if da == 9 && db == 1 {
        "9 10".to_string()
    } else {
        "-1".to_string()
    };

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
