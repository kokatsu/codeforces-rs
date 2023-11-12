use std::io::{stdout, Write, BufWriter};

fn dfs(num: u64, sum: u64, cnt: u64, ret: &mut Vec<u64>) {
    for i in 0..10 {
        if num == 0 && i == 0 {
            continue;
        }
        if sum + i > 10 {
            break;
        }
        if sum + i == 10 {
            ret.push(num*10+i);
        }
        if cnt < 7 {
            dfs(num*10+i, sum+i, cnt+1, ret);
        }
    }
}

fn main() {
    let k: usize = read();

    let mut list: Vec<u64> = Vec::new();

    dfs(0, 0, 0, &mut list);

    list.sort();

    let res: u64 = list[k-1];

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