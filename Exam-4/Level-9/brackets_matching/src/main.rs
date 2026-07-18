fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {

        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}",args);
    for arg in args.iter() {
        if is_balanced(arg) {
            println!("OK");
        } else {
            println!("Error");
        }
    }
}