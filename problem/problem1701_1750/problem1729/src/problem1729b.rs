use std::io::{stdout, Write, BufWriter};

fn main() {
    let t: usize = read();

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..t {
        let n: usize = read();
        let s: Vec<u32> = read_string().chars().map(|c| c as u32 - '0' as u32).collect();

        let mut i: isize = n as isize - 1;
        let mut v: Vec<char> = Vec::new();
        while i >= 0 {
            if s[i as usize] == 0 {
                v.push(char::from_u32(s[i as usize - 2]*10+s[i as usize - 1]+'a' as u32-1).unwrap());
                i -= 3;
            }
            else {
                v.push(char::from_u32(s[i as usize]+'a' as u32-1).unwrap());
                i -= 1;
            }
        }

        v.reverse();

        let res: String = v.into_iter().collect();

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