pub fn prev_prime(nbr: u64) -> u64  {
let mut before:u64 = 0;

let mut i = 2;
while i<nbr{
let  curr = i;
let mut  prime = false;
let mut j = 2;
while j <curr{
if curr % j == 0{

    prime = true ;
    break
}
j+=1;
}
if !prime {

    before = curr;
  
    
} 
i+=1;


}
before




}







// pub fn is_prime(n: u64) -> bool {
//     for i in 2..n {
//         if n % i == 0 {
//             return false;
//         }
//     }
//     true
// }

// pub fn prev_prime(nbr: u64) -> u64 {
//     if nbr < 2 {
//         return 0;
//     }
//     let mut prev = nbr - 1;
//     while prev > 1 {
//         if is_prime(prev) {
//             return prev;
//         }
//         prev -= 1;
//     }
//     0 
// }