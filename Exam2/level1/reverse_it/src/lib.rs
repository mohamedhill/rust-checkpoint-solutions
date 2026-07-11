pub fn reverse_it(v: i32) -> String {
  
    if v < 0 {
        format!("-{}{}", v.abs().to_string().chars().rev().collect::<String>(),v.abs().to_string())
    } else {

        format!("{}{}",v.to_string().chars().rev().collect::<String>(),v.to_string())
    }
}