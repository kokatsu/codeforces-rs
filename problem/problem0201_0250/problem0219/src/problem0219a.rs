use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let k: usize = read();
    let s: Vec<char> = read_string().chars().collect();

    let map: HashMap<char, usize> = s.iter().fold(HashMap::new(), |mut map, &c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let (t, is_ok): (String, bool) =
        map.iter()
            .fold((String::new(), true), |(t, is_ok), (key, val)| {
                if is_ok && val % k == 0 {
                    (t + &key.to_string().repeat(val / k), is_ok)
                } else {
                    (t, false)
                }
            });

    let res: String = if is_ok { t.repeat(k) } else { "-1".to_string() };

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
