pub fn is_digit(character: char) -> bool {
    character.is_digit(10)
}

pub fn is_alpha(character: char) -> bool {
    character.is_alphabetic()
}

pub fn is_alphanumeric(character: char) -> bool {
    character.is_alphanumeric()
}

pub fn parse_string(string: &str) -> Option<f64> {
    string.parse::<f64>().ok()
}
