pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }

    let (mut n, mut i) = (factorial, 2);

    while n % i == 0 {
        n /= i;
        i += 1;
    }

    if n == 1 { i - 1 } else { 0 }
}



// pub fn count_factorial_steps(factorial: u64) -> u64 {
//     if factorial == 0 || factorial == 1 {
//         return 0;
//     }

//     let mut count = 0;
//     let mut product = 1;

//     while product < factorial {
//         count += 1;
//         product *= count;
//     }

//     if product == factorial {
//         count
//     } else {
//         0
//     }
// }



