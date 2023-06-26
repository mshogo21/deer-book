//https://atcoder.jp/contests/abc164/tasks/abc164_b
use proconio::input;

fn main() {
    input!{
        mut a_hp: usize,
        b_at: usize,
        mut c_hp: usize,
        d_at: usize
    }

    while a_hp > 0 && c_hp > 0 {
        if c_hp <= b_at {
            println!("Yes");
            return;
        }
        c_hp -= b_at;
        if a_hp <= d_at {
            println!("No");
            return;
        }
        a_hp -= d_at;
    }
}
