use proconio::input;

fn calc_sum_digits(mut n: usize) -> usize {
    let mut sum_digit = 0;
    while n > 0 {
        sum_digit += n % 10;
        n /= 10;
    }
    sum_digit
}
fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
    }

    let mut result = 0;

    for i in 1..=n {
        let mut _x = calc_sum_digits(i);

        if a <= _x && _x <= b {
            result += i;
        }
    }

    println!("{}",result);
}
