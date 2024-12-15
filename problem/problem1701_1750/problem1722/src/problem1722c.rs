use std::io::{stdout, Write, BufWriter};
use std::collections::HashMap;

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<Vec<String>> = (0..3).map(|_| read_vec::<String>()).collect();

        let map: HashMap<&String, i64> = (0..n*3).fold(HashMap::new(), |mut map, i| {
            *map.entry(&s[i/n][i%n]).or_insert(0) += 1;
            map
        });

        let points: Vec<i64> =
            (0..3)
            .map(|i| {
                s[i]
                .iter()
                .fold(0, |point, x| {
                    if *map.get(&x).unwrap() == 1 {
                        point + 3
                    }
                    else if *map.get(&x).unwrap() == 2 {
                        point + 1
                    }
                    else {
                        point
                    }
                })
            })
            .collect();

        let res: String =
            points
            .iter()
            .fold(String::new(), |res, x| res + &x.to_string() + " ");

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