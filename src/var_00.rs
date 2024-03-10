use std::io::{self, BufRead};

const MAXN: usize = 2501;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    stdin.lock().read_line(&mut buffer).unwrap();
    let arr: Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut dp = vec![0; n];
    let mut answer = 0;

    for i in 0..n {
        let mut remain = arr[i];
        dp[i] = dp[i].max(if i > 0 { dp[i - 1] } else { 0 });
        for j in (i + 1)..n {
            remain = arr[j] - remain;
            if remain < 0 {
                break;
            } else if remain == 0 {
                dp[j] = dp[i - 1] + 1;
            }
        }
    }

    println!("{}", n - dp[n - 1]);
}
