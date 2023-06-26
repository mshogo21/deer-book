//https://atcoder.jp/contests/abc165/tasks/abc165_b
use proconio::input;

fn main() {
    input!{
        x: usize,
    }

    let mut m = 100;
    let mut ans = 0;
    
    while x > m {
        m += m / 100;
        ans += 1;
    }

    println!("{}",ans);
}

