pub fn negative_spell(n: i64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    if n > 0 {
        return "error: positive number".to_string();
    }

    format!("minus {}", number_to_words(-n))
}

fn number_to_words(n: i64) -> String {
    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 0 {
        return "zero".to_string();
    }

    if n >= 1_000_000 {
        let mut s = format!("{} million", number_to_words(n / 1_000_000));
        if n % 1_000_000 != 0 {
            s.push(' ');
            s.push_str(&number_to_words(n % 1_000_000));
        }
        return s;
    }

    if n >= 1_000 {
        let mut s = format!("{} thousand", number_to_words(n / 1_000));
        if n % 1_000 != 0 {
            s.push(' ');
            s.push_str(&number_to_words(n % 1_000));
        }
        return s;
    }

    if n >= 100 {
        let mut s = format!("{} hundred", below_20[(n / 100) as usize]);
        if n % 100 != 0 {
            s.push(' ');
            s.push_str(&number_to_words(n % 100));
        }
        return s;
    }

    if n >= 20 {
        let mut s = tens[(n / 10) as usize].to_string();
        if n % 10 != 0 {
            s.push('-');
            s.push_str(below_20[(n % 10) as usize]);
        }
        return s;
    }

    below_20[n as usize].to_string()
}
#[test]
fn test_short_numbers() {
    assert_eq!(negative_spell(0), "zero");
    assert_eq!(negative_spell(-1), "minus one");
    assert_eq!(negative_spell(-14), "minus fourteen");
    assert_eq!(negative_spell(-20), "minus twenty");
    assert_eq!(negative_spell(-22), "minus twenty-two");
    assert_eq!(negative_spell(-101), "minus one hundred one");
    assert_eq!(negative_spell(-120), "minus one hundred twenty");
    assert_eq!(negative_spell(-123), "minus one hundred twenty-three");
}

#[test]
fn test_medium_numbers() {
    assert_eq!(negative_spell(-1000), "minus one thousand");
    assert_eq!(negative_spell(-1055), "minus one thousand fifty-five");
    assert_eq!(
        negative_spell(-1234),
        "minus one thousand two hundred thirty-four"
    );
    assert_eq!(
        negative_spell(-10123),
        "minus ten thousand one hundred twenty-three"
    );
}

#[test]
fn test_long_numbers() {
    assert_eq!(
        negative_spell(-910112),
        "minus nine hundred ten thousand one hundred twelve"
    );
    assert_eq!(
        negative_spell(-651123),
        "minus six hundred fifty-one thousand one hundred twenty-three"
    );

    assert_eq!(negative_spell(-810000), "minus eight hundred ten thousand");
    assert_eq!(negative_spell(-1000000), "minus one million");
}

#[test]
fn test_invalid_numbers() {
    assert_eq!(negative_spell(1), "error: positive number");
    assert_eq!(negative_spell(2390904), "error: positive number");
}