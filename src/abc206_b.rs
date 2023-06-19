use proconio::input;

fn main() {
    input!{
        n: usize
    }

    let mut money = 0;
    let mut days = 1;

    while money < n {
        money += days;
        days += 1;
    }

    println!("{}",days - 1);
}
