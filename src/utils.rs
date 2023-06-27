pub fn round_to_digit(value: f64, digit_number: usize) -> usize {
    let int_value = value as usize;
    let v_string = format!("{}", int_value);
    let number_of_digits = v_string.len();
    if number_of_digits == 1 {
        return int_value + 1;
    };
    let mut res = 0;
    for (i, c) in v_string.chars().enumerate() {
        let power = (number_of_digits - i - 1) as u32;
        if i + 1 == digit_number {
            res += (c.to_digit(10).unwrap() as usize + 1) * 10_usize.pow(power);
            return res;
        } else {
            res += (c.to_digit(10).unwrap() as usize) * 10_usize.pow(power);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rount_to_digit() {
        let res = round_to_digit(31323.1, 2);
        assert_eq!(res, 32000);
        let res = round_to_digit(3999.1, 2);
        assert_eq!(res, 4000);
        let res = round_to_digit(39.1, 3);
        assert_eq!(res, 39);
        let res = round_to_digit(39.1, 2);
        assert_eq!(res, 40);
        let res = round_to_digit(39.1, 1);
        assert_eq!(res, 40);
    }
}
