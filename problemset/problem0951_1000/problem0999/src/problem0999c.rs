use std::io::{stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let (_n, k): (usize, usize) = {
        let input = read_vec::<usize>();
        (input[0], input[1])
    };

    let s: Vec<char> = read_string().chars().collect();

    let mut t: Vec<char> = s.clone();
    t.sort();

    let mut a: Vec<usize> = vec![0; 26];
    for &c in t.iter().take(k) {
        a[c as usize - 'a' as usize] += 1;
    }

    let res: String = s.iter().fold(String::new(), |res, &c| {
        let x: usize = c as usize - 'a' as usize;
        if a[x] > 0 {
            a[x] -= 1;
            res
        } else {
            res + &c.to_string()
        }
    });

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
