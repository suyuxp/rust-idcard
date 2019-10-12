pub fn validate(idcard: &str) -> bool {
    idcard.len() == 18
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(validate("510108197205052137"), true);
    }
}
