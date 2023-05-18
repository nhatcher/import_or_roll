// imports Math.sin from the host
#[link(wasm_import_module = "Math")]
extern {
    fn sin(x: f64) -> f64;
}

// 
#[no_mangle]
pub extern fn count_import(n: f64, t: f64) -> i32 {
    let mut count = 0;
    let mut x: f64 = 0.0;
    while x < n {
        if unsafe { sin(x) } < t {
            count += 1;
        }
        x += 1.0;
    }
    count
}

#[no_mangle]
pub extern fn count_roll(n: f64, t: f64) -> i32 {
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