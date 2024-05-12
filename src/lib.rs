pub fn number(user_number: &str) -> Option<String> {
    // Remove all non-digit characters
    let cleaned_number: String = user_number.chars().filter(|c| c.is_digit(10)).collect();

    //  if the cleaned number is empty or contains less than 10 digits
    if cleaned_number.len() < 10 {
        return None;
    }

    // if the cleaned number starts with '1' and remove it
    let cleaned_number_without_country_code = if cleaned_number.starts_with('1') {
        cleaned_number[1..].to_string()
    } else {
        cleaned_number.clone()
    };

    // if the resulting number contains exactly 10 digits
    if cleaned_number_without_country_code.len() != 10 {
        return None;
    }

    // if the area code and exchange code are valid
    let area_code = &cleaned_number_without_country_code[..3];
    let exchange_code = &cleaned_number_without_country_code[3..6];

    // Area code and exchange code cannot start with '0' or '1'
    if [area_code, exchange_code].iter().any(|&code| code.starts_with('0') || code.starts_with('1')) {
        return None;
    }

    Some(cleaned_number_without_country_code)
}
