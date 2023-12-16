pub fn is_digit(character: char) -> bool {
    return character.is_digit(10);
}

pub fn parse_string(string: &str) -> Option<f64> {
    string.parse::<f64>().ok()
}
