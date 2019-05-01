pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();

    num == num_string
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .map(|v| v.pow(num_string.len() as u32))
        .sum()
}
