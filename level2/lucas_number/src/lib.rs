pub fn lucas_number(n: u32) -> u32 {
    if n == 0 {
        2
    } else if n == 1 {
        1
    } else {
        lucas_number(n - 1) + lucas_number(n - 2)
    }
}