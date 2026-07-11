pub fn rot21(input: &str) -> String {
let mut  res = String::new();

for i in input.chars(){

if i.is_ascii_lowercase(){

let new = ((((i as u8 - b'a')+21)%26)+b'a') as char;
res.push(new);

}else if i.is_ascii_uppercase(){
let new = ((((i as u8 - b'A')+21)%26)+b'A') as char;
res.push(new);

}else{
    res.push(i)
}


}
res

}