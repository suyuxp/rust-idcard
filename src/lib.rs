pub fn validate(idcard: &str) -> bool {
    let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let sum: u32 = idcard
        .chars()
        .take(17)
        .zip(weights.iter())
        .map(|(d, w)| d.to_digit(10).unwrap_or(10) * w)
        .sum();

    let code = match sum % 11 {
        0 => '1',
        1 => '0',
        2 => 'X',
        3 => '9',
        4 => '8',
        5 => '7',
        6 => '6',
        7 => '5',
        8 => '4',
        9 => '3',
        10 => '2',
        _ => ' ',
    };

    match idcard.chars().last() {
        Some(v) => code == v.to_ascii_uppercase(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::validate("510108197205052137"), true);
        assert_eq!(super::validate("15040419840217262X"), true);
        assert_eq!(super::validate("15040419840217262x"), true);
        assert_eq!(super::validate("150404198402172620"), false);
    }
}
