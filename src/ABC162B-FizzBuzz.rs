use proconio::input;

fn main() {
    input!{
        n: usize
    }

    let mut ans = 0;
    
    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            ans += i;
        }
    }

    println!("{}",ans);
}
