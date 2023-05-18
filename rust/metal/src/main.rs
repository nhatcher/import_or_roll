pub extern fn count_numbers(n: f64, t: f64) -> i32 {
    let mut count = 0;
    let mut x: f64 = 0.0;
    while x < n {
        if f64::sin(x) < t {
            count += 1;
        }
        x += 1.0;
    }
    count
}


fn main() {
    let n = 100_000_000.0;
    let t = 0.223;
    println!("{}", count_numbers(n ,t));
}
