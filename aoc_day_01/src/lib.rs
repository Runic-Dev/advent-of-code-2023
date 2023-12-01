use std::fs;

pub fn parse_callibration() -> usize {
    todo!();
}

fn read_calibration_file() -> String {
    match fs::read_to_string("input.txt") {
        Ok(read_file) => read_file,
        Err(err) => panic!("Couldn't read from file: {}", err),
    }
}

fn parse_first_number(input: &str) -> usize {
    for char in input.chars() {
        if char.is_numeric() {
            return char.to_digit(10).unwrap() as usize;
        }
    }

    panic!("No number found");
}

#[cfg(test)]
mod calibration_parser_should {
    use super::*;

    #[test]
    fn return_1_when_first_number_is_1() {
        let input = String::from("abc1def");
        let first_number = parse_first_number(&input);
        assert_eq!(1, first_number);
    }

    #[test]
    fn return_2_when_first_number_is_2() {
        let input = String::from("abc2def");
        let first_number = parse_first_number(&input);
        assert_eq!(2, first_number);
    }

    #[test]
    fn return_3_when_first_number_is_3() {
        let input = String::from("abc3def");
        let first_number = parse_first_number(&input);
        assert_eq!(3, first_number);
    }
    #[test]
    fn return_4_when_first_number_is_4() {
        let input = String::from("abc4def");
        let first_number = parse_first_number(&input);
        assert_eq!(4, first_number);
    }
}

