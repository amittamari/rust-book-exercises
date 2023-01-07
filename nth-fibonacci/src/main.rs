fn main() {
    println!("{}", get_nth_fibonacci(6));
}

fn get_nth_fibonacci(n: u32) -> u32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2)
    }
}
