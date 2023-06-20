//https://atcoder.jp/contests/abc200/tasks/abc200_b
use proconio::input;

fn main() {
    input!{
        mut n: i64,
        k: i64,
    }

    for _i in 1..=k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = 1000 * n + 200;
        }
    }
    println!("{}",n);
}
