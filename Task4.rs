use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums_iter = input.split_whitespace();
    let n: usize = nums_iter.next().unwrap().parse().unwrap();
    let m: usize = nums_iter.next().unwrap().parse().unwrap();

    let mut nums = vec![0; n];
    for i in 0..n {
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).unwrap();
        nums[i] = num_input.trim().parse().unwrap();
    }

    let mut dp = vec![-1; n + 1];
    let mut prefix_sum = vec![0; n + 1];

    for i in 1..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    dp[n] = 0;
    for i in (0..n).rev() {
        let max_numbers = min(m, n - i);
        for j in 1..=max_numbers {
            let sum = prefix_sum[i + j] - prefix_sum[i];
            dp[i] = dp[i].max(sum - dp[i + j]);
        }
    }

    if dp[0] > 0 {
        println!("1");
    } else {
        println!("0");
    }
}