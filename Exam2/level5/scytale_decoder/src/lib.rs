pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let cols = letters_per_turn as usize;

    let rows = (len + cols - 1) / cols;

    let mut result = String::with_capacity(len);

    for col in 0..cols {
        for row in 0..rows {
            let idx = row * cols + col;
            if idx < len {
                result.push(chars[idx]);
            }
        }
    }
    Some(result)
}